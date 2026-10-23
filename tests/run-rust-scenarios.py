#!/usr/bin/env python3
"""Run disposable, offline Rust/C boundary fixtures; requires Cargo, cc and ar.

These are reference scenarios for evaluating the skill, not proof that an AI
client follows its instructions. The fixture MSRV is declared but not installed.
"""
from pathlib import Path
import os
import shutil
import subprocess
import tempfile


def run(project, *args, expected_failure=None):
    result = subprocess.run(
        ["cargo", *args, "--offline"], cwd=project, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        env={**os.environ, "CARGO_TERM_COLOR": "never"}, timeout=120,
    )
    print("$ cargo", *args, "--offline", flush=True)
    print(result.stdout, end="", flush=True)
    if expected_failure:
        assert result.returncode != 0 and expected_failure in result.stdout, (
            "Expected the named behavior/contract failure, not a setup error"
        )
    else:
        assert result.returncode == 0, "Cargo scenario failed"


def main():
    for tool in ("cargo", "cc", "ar"):
        if shutil.which(tool) is None:
            raise SystemExit(f"Required fixture tool unavailable: {tool}")
    source = Path(__file__).resolve().parent / "fixtures/rust-boundaries"
    with tempfile.TemporaryDirectory(prefix="rust-skill-scenarios-") as temporary:
        project = Path(temporary) / "fixture"
        shutil.copytree(source, project)
        library = project / "src/lib.rs"
        original = library.read_text()
        # A real assertion fails before each isolated correction; never run UB.
        library.write_text(original.replace("if value == 0 {", "if value == 0 || value == u16::MAX {"))
        run(project, "test", "--lib", "parser_normal_invalid_and_boundaries",
            expected_failure="assertion `left == right` failed")
        library.write_text(original.replace("self.0.set(false);", "// missing cancellation cleanup"))
        run(project, "test", "--lib", "cancellation_after_progress_releases_state_before_shutdown",
            expected_failure="shutdown would leave a reservation behind")
        library.write_text(original)
        run(project, "test")  # runtime cases and !Send compile-fail doctest
        run(project, "check", "--no-default-features", "--features", "backend-a")
        run(project, "check", "--no-default-features", "--features", "backend-b")
        run(project, "check", "--all-features",
            expected_failure="backend-a and backend-b are mutually exclusive")
        helper = Path(__file__).resolve().parents[1] / "scripts/check.sh"
        subprocess.run(["bash", str(helper), str(project)], check=True, timeout=120,
                       env={**os.environ, "CARGO_NET_OFFLINE": "true"})
    print("PASS: runtime, behavioral RED/GREEN, foreign allocation, !Send, host no_std and feature contracts")
    print("LIMIT: no AI-client behavior, MSRV run, embedded target, callback protocol, Miri or exhaustive scheduling proof")


if __name__ == "__main__":
    main()
