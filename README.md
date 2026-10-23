# Google Rust Style — Agent Skill

An independent Agent Skills-compatible coding skill adapted from:
https://google.github.io/styleguide/rustguide.html

Includes:

- `SKILL.md`: invocation and operating instructions.
- `references/rules.md`: summarized rules, scope, and exceptions.
- `references/examples.md`: before/after idiomatic Rust examples.
- `references/review-checklist.md`: structured quality gate.
- `references/async-concurrency.md`, `unsafe-ffi.md`, `validation.md`: conditional
  engineering guidance for cancellation, safety boundaries, and validation.
- `scripts/check.sh`: optional `cargo fmt`, Clippy, and test verification.

It is an independent adaptation and is not published or endorsed by Google.

## Install

### Codex and Claude Code with the Skills CLI

Requires Node.js/npm (`npx`) and network access. Run from the project where
you want the skill available:

```bash
npx skills add chuhemiao/google-rust-style --skill google-rust-style -a codex -a claude-code
```

To install for your user account across projects, add `-g`:

```bash
npx skills add chuhemiao/google-rust-style --skill google-rust-style -a codex -a claude-code -g
```

To preview available skills before installing:

```bash
npx skills add chuhemiao/google-rust-style --list
```

The [Skills CLI](https://github.com/vercel-labs/skills/blob/main/docs/commands.md)
supports additional agents through its interactive agent selector. Omit the
`-a` options to choose your installed tools. Use `--copy` if you prefer copies
instead of the CLI's default symlink installation. These commands install
the version published on GitHub. To use an unpublished local checkout,
follow the manual installation instructions below.

### Manual installation

Copy this repository's `SKILL.md`, `references/`, and `scripts/` together into
a directory named `google-rust-style` under one of these locations:

| Tool | Current project | User account |
| --- | --- | --- |
| [Codex](https://developers.openai.com/codex/skills) | `.agents/skills/google-rust-style/` | `~/.agents/skills/google-rust-style/` |
| [Claude Code](https://code.claude.com/docs/en/skills) | `.claude/skills/google-rust-style/` | `~/.claude/skills/google-rust-style/` |
| [Cursor](https://cursor.com/docs/skills) | `.cursor/skills/google-rust-style/` | `~/.cursor/skills/google-rust-style/` |
| [GitHub Copilot CLI](https://docs.github.com/en/copilot/how-tos/copilot-cli/customize-copilot/add-skills) | `.github/skills/google-rust-style/` | `~/.copilot/skills/google-rust-style/` |

The resulting layout must keep `SKILL.md` directly inside that directory:

```text
google-rust-style/
├── SKILL.md
├── references/
│   ├── rules.md
│   ├── examples.md
│   ├── review-checklist.md
│   ├── async-concurrency.md
│   ├── unsafe-ffi.md
│   └── validation.md
└── scripts/
    └── check.sh
```

Keep the relative paths intact. Avoid installing a second copy into the same
scope; update the existing installation when upgrading. Other tools with
Agent Skills support can use the same folder according to their own discovery
rules; no Codex or Claude-specific plugin is required.
User account paths above configure local tools, not cloud agent environments.

## Use

Ask your agent to use `google-rust-style` to write, refactor, or review Rust.
In Codex, invoke `$google-rust-style`; in Claude Code, invoke
`/google-rust-style`. For example:

> Use google-rust-style to implement this Rust API. Preserve the project's
> MSRV and feature support, and run the relevant checks.

The skill distinguishes ordinary Cargo projects from Chromium and Fuchsia.
It reads existing project conventions before applying guide preferences.

## Optional Rust checks

From any working directory, pass your Cargo project to the installed helper:

```bash
bash /path/to/google-rust-style/scripts/check.sh /path/to/cargo-project
```

Requires Bash, Cargo, rustfmt, and Clippy. The helper checks formatting,
runs Clippy with warnings denied, then runs workspace tests. Use
`--skip-clippy` or `--skip-test` when appropriate. It stops on the first
failure and preserves that command's exit code.

Cargo can run project build scripts and tests, fetch dependencies, update
`Cargo.lock`, and create build artifacts. Follow the project's own CI commands
for feature, target, MSRV, locked, or offline validation. On Windows, run the
helper in an environment with Bash, or use the Cargo commands in `SKILL.md`
directly.

## Sources and scope

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html)
  supplies the summarized rules and platform exceptions.
- [Rust Style Guide](https://doc.rust-lang.org/style-guide/) and
  [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) supply
  complementary conventions.

The references summarize the sources rather than embedding the source guides.
Project requirements take precedence; review findings distinguish source
guidance from repository conventions and optional engineering advice.

## Maintaining and evaluating this skill

Use [behavioral acceptance scenarios](tests/behavioral-acceptance.md) to evaluate
changes with realistic Rust tasks. They test decisions and observed behavior,
not exact response wording. New references are skill-added engineering guidance,
not rules attributed to Google.

Package/link validation establishes that the copied resources are complete. It
does not establish client discovery, automatic activation, or successful task
execution. Record the client/version, installed scope, invocation and observed
resource loading when testing Codex or Claude Code. CLI installation success
alone does not prove runtime behavior, and one client does not prove another.

Run `python3 tests/run-rust-scenarios.py` for disposable offline Rust/C boundary
checks (Python 3, Cargo components, `cc` and `ar` required). See
[client validation](tests/client-validation.md) for actual client versions,
discovery evidence, authentication blockers and repeatable probes. Rust fixture
success does not establish model behavior.
