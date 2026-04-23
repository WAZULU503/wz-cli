# wz-cli

Fix critical AI code mistakes before you run them.  
Preview the exact change. Apply safely.

Run this before executing AI-generated code.  
Use it after copying AI-generated code, before running it.

No account. No internet. One command.

---

## The Problem

AI writes code fast. It also makes the same mistakes, fast.

Unsafe dictionary access. Undefined variables. Constant conditions that never change.

You don’t always catch them before you run the file.

---

## What wz-cli Does

Scans your file, shows the exact diff, and applies changes only with your confirmation.

```
- print(data["b"])
+ print(data.get("b"))

Apply changes? (y/n):
```

No surprises. No blind rewrites. You stay in control.

Designed for quick checks, not full static analysis.

---

## Install

```bash
cargo install --path .
```

Run from your project directory.

---

## Usage

**Check a file for issues:**
```bash
wz-cli verify file.py
```

**Preview and apply fixes:**
```bash
wz-cli apply file.py
```

**Preview only — no write, no prompt:**
```bash
wz-cli apply file.py --dry
```

---

## Example

```python
# file.py
data = {"a": 1}
print(data["b"])
```

```bash
$ wz-cli apply file.py
```

```
===== DIFF =====

-     print(data["b"])
+     print(data.get("b"))

================

Apply changes? (y/n):
```

Only critical fixes are applied automatically. Everything else is left for you to review.

---

## Rules

| Rule | What it catches | Auto-fix |
|------|----------------|----------|
| R2 | Unsafe dictionary access `data["key"]` | ✅ Yes |
| R3 | Returning undefined variable | ✅ Yes |
| R1 | Unreachable condition | ⚠️ Suggest only |
| R4 | Excessive nesting | ⚠️ Suggest only |
| R5 | Constant condition `if True` | ⚠️ Suggest only |

---

## One Rule

**Always read the diff before you hit `y`.**

wz-cli shows you exactly what will change. That’s the whole point. Use it.

---

## What It's Not

Not a full linter. Not an AST parser. Not a cloud service.

It’s a fast pre-execution gate for the common mistakes AI makes on simple code.

v1. Honest. Does what it says.

---

## License

MIT
