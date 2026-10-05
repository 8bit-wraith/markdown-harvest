# Baseline quality gates

Status: Implement, local only

The HTTP configuration repair passes functional tests but six existing clippy findings and whole-project formatting block the defined quality gates. Make behavior-preserving simplifications, then reconcile UserAgent string conversion without silently breaking public usage. Preserve user-agent literals, selector priority, chunk-overlap rejection and feature gates. No lint suppression, dependency changes or release dispatch.

Acceptance: all-feature locked tests/build, warnings-denied clippy and formatting pass; preserve HTTP regression coverage and public usage. Two workers, existing cache, each command bounded to180s. Cross-platform validation remains separate.

Checkpoint: simplified five baseline lint findings without changing behavior. Clippy now reports only UserAgent inherent_to_string. Full tests reproduced an async cookie fixture WouldBlock race twice; accepted sockets now explicitly restore blocking mode before bounded read timeout in both fixtures. All65 tests plus18 doctests pass (one external test ignored). Whole-project format and string conversion remain pending. No commit/push or running jobs.

Local completion: Display now provides standard ToString; all12 exact browser strings, method calls and UserAgent::to_string function-pointer usage are covered. Five other lint corrections and rustfmt-only baseline cleanup included. All-feature locked/offline tests (66 passed,1ignored;18doctests), build, warnings-denied clippy and whole-project format pass. Cross-platform hosted checks remain pending; no release.
