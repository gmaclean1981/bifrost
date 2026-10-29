# Maintenance changes

Version `dev-2026-10-28-maint.1`; modifications dated October 28, 2026.
Base: duvholt/bifrost `c75be22a54d45495918bfe0751844363f7c9fc6d`.

1. Retain the `mdns-sd` 0.13.2 query-handler correction from
   `if !out.answers_count() > 0` to `if out.answers_count() > 0`.
2. Retain five UDP regression tests in `mdns-sd/src/mdns_patch_tests.rs`.
3. Select that vendored crate through `[patch.crates-io]`; exclude its independent
   test workspace from the root workspace.
4. Retain compatible dependency updates: bytes 1.11.1, h2 0.4.16, anyhow 1.0.103,
   rand 0.8.6 and 0.9.3, tokio 1.53.3. Other upstream versions remain as pinned.
5. Add portable build/test instructions, attribution and release evidence.
6. Keep inherited GitHub workflows manual-only to avoid automatically publishing
   unvalidated images from this personal fork.

No Bifrost application logic was changed from the pinned upstream revision.
The executable checksum is `60318ae4bbb9316428a435fabf6b8b66568e83faa6acfb87d7c3bf0404198115`.

## AI-agent disclosure

The maintenance modifications, regression tests, build scripts, documentation,
and Home Assistant packaging in this fork were coded using an AI coding agent
(OpenAI Codex), with direction and hands-on device testing by the repository
owner. Bifrost itself and its upstream features were created by their original
authors and contributors; this disclosure does not attribute their work to AI.
