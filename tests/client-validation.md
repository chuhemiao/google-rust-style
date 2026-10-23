# Client validation

## Observed run: 2026-10-23

Tested local package copies in disposable projects, using existing account auth
without changing user installations or configuration. The project under review
was outside the installed skill directory. No model-generated code was executed. Rust checks used rustc/Cargo 1.95.0,
Clippy 0.1.95, rustfmt 1.9.0 and Apple clang 21 on aarch64-apple-darwin.

| Boundary | Actual result |
| --- | --- |
| Codex CLI 0.142.5 project discovery | `codex app-server` → `skills/list` returned `google-rust-style`, the copied `.agents/skills/google-rust-style/SKILL.md`, `scope: repo`, `enabled: true`, and no discovery errors |
| Codex model execution | `codex exec` failed before resource reads: default `gpt-5.5`, the existing configured `gpt-6.1-sol`, and a model advertised by this client's `model/list` (`gpt-5.4-mini`) were rejected as unsupported with this ChatGPT account |
| Claude Code 2.1.220 project discovery | With `--setting-sources project`, stream-JSON initialization listed `google-rust-style` in both `skills` and `slash_commands` |
| Claude model execution | Failed before a `Skill`/`Read` event: `OAuth session expired and could not be refreshed` |
| User-scope installation, automatic selection, conditional reference reads | Not verified; project discovery is insufficient evidence |
| Installed helper | Executed from outside the installation against the disposable Rust fixture; fmt, Clippy and tests passed |

No skill runtime defect was demonstrated. Both clients recognized the project
package, but authenticated inference must succeed before either client's task
behavior can be accepted. Local `login status` alone did not prove inference
worked. Reauthenticate with `codex login` / `claude auth login` as appropriate,
then repeat using a model available to that account. These commands change auth
and were not run as part of the evaluation. Client updates were not run. A later attempt to enumerate installed Rust
toolchains/targets with `rustup` unexpectedly began a stable toolchain download;
it was interrupted (exit 130). Read-only inspection found a newly modified
partial `~/.rustup/toolchains/stable-aarch64-apple-darwin` directory without
`bin/rustc`, plus download/temp artifacts; `settings.toml` retained its May 30
mtime. These global artifacts were left untouched. No completed toolchain
installation or additional MSRV/embedded-target validation is claimed. The scenario runner itself uses
existing tools and never invokes `rustup`.

A failed initial Claude probe used `--setting-sources ''`, which excludes the
project skill scope and returned `Unknown command`. Keep `project` when testing
`.claude/skills`; this was a harness correction, not a skill defect.

## Repeat the client checks

Copy `SKILL.md`, `references/` and `scripts/` into the client's project skill
location in a disposable Cargo project. Do not overwrite user installations.
For an ordinary parser task, explicit invocation can be tested with:

```bash
codex exec --ignore-user-config --ignore-rules --ephemeral \
  --skip-git-repo-check --sandbox read-only -C /path/to/disposable-project \
  --model YOUR_AVAILABLE_MODEL --json \
  'Use $google-rust-style to review src/lib.rs. Do not edit. Read only relevant references and report the installed helper path.'

# Run from the disposable project containing .claude/skills/google-rust-style.
claude -p '/google-rust-style Review src/lib.rs. Do not edit. Read only relevant references and report the installed helper path.' \
  --setting-sources project --strict-mcp-config --mcp-config '{"mcpServers":{}}' \
  --tools Read,Glob,Grep,Skill --allowedTools Read,Glob,Grep,Skill \
  --permission-mode dontAsk --no-session-persistence \
  --output-format stream-json --verbose
```

Use the installed CLI's current help for version-specific flags. Inspect actual
`Skill`/file-read/command events and final judgments, not merely the client's
claim to have loaded a skill. Repeat with async/unsafe fixtures to check
conditional resources; repeat without naming the skill for automatic selection.
Run writable implementation cases only inside the disposable project with
appropriate tool permissions. The read-only probes above evaluate reviews.

`--ignore-user-config` skips Codex configuration, not every source of skills or
cached state. `CODEX_HOME` changes auth/config/cache locations, but Codex user
skills still live under `$HOME/.agents/skills`. This run used a temporary
`CODEX_HOME` with an auth symlink; it did not claim user-scope isolation.
Claude's settings-source selection likewise is not a general filesystem sandbox.
Inspect the actual selected path to distinguish the tested copy from other
installed skills. Remove temporary copies and auth symlinks after evaluation.

Sources: [Codex skills](https://learn.chatgpt.com/docs/build-skills),
[Codex CLI](https://learn.chatgpt.com/docs/developer-commands?surface=cli),
[Codex app-server](https://learn.chatgpt.com/docs/app-server),
[Claude skills](https://code.claude.com/docs/en/skills),
[Claude CLI](https://code.claude.com/docs/en/cli-reference).
