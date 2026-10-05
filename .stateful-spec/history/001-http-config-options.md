# HTTP configuration options

Status: Implement / reproduce

Both client constructors discard redirect and cookie settings when timeout is unset. Keep options independent, retain current default redirect behavior (10 without timeout, 2 with timeout), and preserve public signatures.

Acceptance: synthetic loopback redirect and cookie tests for blocking and async clients, with and without timeout. Run all-feature tests/build, clippy warnings denied, and format check; record existing failures separately. No external requests in new tests.

Resources: at most two Cargo workers, 180-second initial build limit; estimated under 2 GB build artifacts and 4 GB RAM. Observed 3.3 TiB free disk, 86% memory free, no cargo/rustc processes. No repository build script. Dependencies are the existing locked crates.io packages.

Checkpoint: cookie regressions failed on original code for both clients. Constructors now apply independent options and fail explicitly on builder failure instead of silently reverting settings. Four loopback regressions pass, full suite 65 passed / 1 ignored plus 18 doctests, all-feature build passes. Clippy reports six existing findings in unchanged source; whole-repository format check also fails. Not complete: add timeout/default cookie controls and reconcile existing quality-gate failures before declaring completion. No jobs running.

2026-10-05 follow-up: cookie enabled/disabled controls now pass for both blocking/async clients with timeout absent and present (eight scenarios), plus two no-timeout redirect-limit scenarios. Focused four test functions pass; git diff --check passes. Functional patch ready for review, not verified-complete: pre-existing whole-project lint/format gates remain failing in untouched code. Avoid unrelated API/format churn in this patch.

Publication review: all-feature offline locked tests and build revalidated on macOS; 65 tests and 18 doctests pass, one external HTTP test ignored. Changed HTTP file passes rustfmt. Whole-project clippy still fails six findings in unchanged files; whole-project fmt fails. Keep publication as a draft until quality gates are reconciled; do not claim completion or publish a crate.
