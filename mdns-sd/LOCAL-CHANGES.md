# Local modifications to mdns-sd 0.13.2

Retained October 28, 2026 for the Bifrost maintenance build by gmaclean1981.
Correct the outgoing answer-count predicate in `src/service_daemon.rs` and add
five isolated UDP regressions in `src/mdns_patch_tests.rs` (included by the daemon
test module). The original crate authorship and MIT/Apache license texts remain.
This crate is distributed under its MIT option. The modifications/tests were
prepared using OpenAI Codex with direction and device testing by the owner.
