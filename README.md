printf '%s
' "# wz-cli

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

\`\`\`
- print(data[\"b\"])
+ print(data.get(\"b\"))

Apply changes? (y/N):
\`\`\`

No surprises. No blind rewrites. You stay in control.

---

## Install

\`\`\`
cargo install --path .
\`\`\`

---

## Usage

\`\`\`
wz-cli verify file.py
wz-cli apply file.py
wz-cli apply file.py --dry
\`\`\`

---

## License

MIT" > README.md
