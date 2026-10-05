# Cross-platform validation

Status: Hosted verification pending

The project definition requires Linux, macOS and Windows support. Existing PR CI tested Linux only; the cross-platform workflow packages/releases binaries and must not be used for routine verification. Expand source-only Build to three standard GitHub runners with two concurrent jobs, two Cargo workers and a fifteen-minute job limit. Use read-only contents permission and locked dependencies. Preserve existing triggers; add this repair branch to push triggers so draft changes can be tested independently of PR registration.

Observed at ba100c59dad69939f64c77dbf0f818efea8e136c: public fork, Actions enabled, all actions allowed, build.yml present on main, but workflow inventory lists only Dependabot and build.yml lookup returns404. No tests have run on hosted Linux/Windows. Acceptance requires successful current-head build/test jobs on all three platforms. No release workflow dispatch or credentials/settings changes.
