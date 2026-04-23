use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
struct Issue {
    rule: String,
    message: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1];
    let file_path = &args[2];

    // detect dry-run flag
    let dry_run = args.iter().any(|a| a == "--dry");

    let code = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("✗ ERROR — file not found: {}", file_path);
            std::process::exit(1);
        }
    };

    let issues = l3_reasoning(&code);

    match command.as_str() {
        "verify" => run_verify(&issues),
        "apply" => run_apply(file_path, &code, &issues, dry_run),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  wz-cli verify <file>");
    println!("  wz-cli apply  <file> [--dry]");
}

fn run_verify(issues: &Vec<Issue>) {
    if issues.is_empty() {
        println!("✓ SAFE — no issues found");
        return;
    }

    let has_critical = issues.iter().any(|i| is_critical(&i.rule));

    if has_critical {
        println!("✗ FAIL — critical issues found\n");
    } else {
        println!("⚠ NEEDS FIX — non-critical issues found\n");
    }

    for (i, issue) in issues.iter().enumerate() {
        let level = if is_critical(&issue.rule) {
            "CRITICAL"
        } else {
            "MAJOR"
        };

        println!("{}. [{}] {}", i + 1, level, issue.message);
        println!("   → {}", propose_fix(&issue.rule));
        println!();
    }
}

fn run_apply(path: &str, original: &str, issues: &Vec<Issue>, dry_run: bool) {
    if issues.is_empty() {
        println!("✓ SAFE — nothing to fix");
        return;
    }

    let modified = apply_fixes(original, issues);

    println!("\n===== DIFF =====\n");
    print_diff(original, &modified);
    println!("\n================\n");

    // DRY RUN MODE — EXIT EARLY
    if dry_run {
        println!("(dry-run) no changes applied");
        return;
    }

    print!("Apply changes? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        if let Err(e) = fs::write(path, &modified) {
            eprintln!("✗ ERROR — could not write file: {}", e);
            std::process::exit(1);
        }
        println!("✓ Changes applied.");
    } else {
        println!("✗ Aborted.");
    }
}

fn print_diff(original: &str, modified: &str) {
    let orig_lines: Vec<&str> = original.lines().collect();
    let mod_lines: Vec<&str> = modified.lines().collect();

    let max_len = std::cmp::max(orig_lines.len(), mod_lines.len());

    for i in 0..max_len {
        let o = orig_lines.get(i).unwrap_or(&"");
        let m = mod_lines.get(i).unwrap_or(&"");

        if o != m {
            println!("- {}", o);
            println!("+ {}", m);
        }
    }
}

fn is_critical(rule: &str) -> bool {
    matches!(rule, "R2_UNSAFE_ACCESS" | "R3_UNDEFINED_VAR")
}

fn propose_fix(rule: &str) -> String {
    match rule {
        "R3_UNDEFINED_VAR" => "Define variable before returning".into(),
        "R2_UNSAFE_ACCESS" => "Use dict.get(key) instead of direct indexing".into(),
        "R1_UNREACHABLE" => "Fix unreachable condition".into(),
        "R4_COMPLEXITY" => "Reduce nesting".into(),
        "R5_CONSTANT_CONDITION" => "Remove constant condition".into(),
        _ => "Manual fix required".into(),
    }
}

fn apply_fixes(code: &str, issues: &Vec<Issue>) -> String {
    let mut modified = code.to_string();

    for issue in issues {
        if !is_critical(&issue.rule) {
            continue;
        }

        match issue.rule.as_str() {
            "R2_UNSAFE_ACCESS" => {
                modified = modified.replace("[\"", ".get(\"");
                modified = modified.replace("\"]", "\")");
            }
            "R3_UNDEFINED_VAR" => {
                if !modified.contains("result =") {
                    modified = format!("result = None\n{}", modified);
                }
            }
            _ => {}
        }
    }

    modified
}

fn l3_reasoning(code: &str) -> Vec<Issue> {
    let mut issues = Vec::new();

    if code.contains("elif") && code.contains("> 10") && code.contains("> 20") {
        issues.push(Issue {
            rule: "R1_UNREACHABLE".into(),
            message: "Possible unreachable condition".into(),
        });
    }

    for line in code.lines() {
        let l = line.trim();
        if l.contains("[\"") && !l.contains("if") {
            issues.push(Issue {
                rule: "R2_UNSAFE_ACCESS".into(),
                message: "Possible unsafe dictionary access".into(),
            });
            break;
        }
    }

    if code.contains("return result") {
        let mut defined = false;
        for line in code.lines() {
            let l = line.trim();
            if l.starts_with("result =") || l.contains("result:") {
                defined = true;
                break;
            }
        }
        if !defined {
            issues.push(Issue {
                rule: "R3_UNDEFINED_VAR".into(),
                message: "Returning undefined variable".into(),
            });
        }
    }

    let indent = code
        .lines()
        .map(|l| l.replace("\t", "    "))
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
        .max()
        .unwrap_or(0);

    if indent > 12 {
        issues.push(Issue {
            rule: "R4_COMPLEXITY".into(),
            message: "Excessive nesting".into(),
        });
    }

    if code.contains("if True") || code.contains("if False") {
        issues.push(Issue {
            rule: "R5_CONSTANT_CONDITION".into(),
            message: "Constant condition detected".into(),
        });
    }

    issues
}
