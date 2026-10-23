# Google Rust Style — Agent Skill

An independent Agent Skills-compatible coding skill adapted from:
https://google.github.io/styleguide/rustguide.html

Includes:
- `SKILL.md`: invocation and operating instructions.
- `references/rules.md`: summarized rules, scope, and exceptions.
- `references/examples.md`: before/after idiomatic Rust examples.
- `references/review-checklist.md`: structured quality gate.
- `scripts/check.sh`: optional `cargo fmt`, Clippy, and test verification.

It is an independent adaptation and is not published or endorsed by Google.

## Install

For Codex, copy the `google-rust-style` directory into the repository's
`.agents/skills/` folder, or a supported user-level skills directory.

For Claude Code, copy the same directory into the repository's
`.claude/skills/` folder, or a supported user-level skills directory.

Then request: "Use google-rust-style to implement/review the Rust code."

Do not flatten `references/` or `scripts/`: paths in `SKILL.md` are relative
to the Skill root.
