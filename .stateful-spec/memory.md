# markdown-harvest — Project Memory

**Status:** Active development

## Project Summary

A Rust crate designed to extract, clean, and convert web content from URLs found in text messages into clean Markdown format. Originally created as an auxiliary component for Retrieval-Augmented Generation (RAG) solutions to process URLs submitted by users. Version 0.1.6.

## Constraints & Reminders

- No unsafe code without justification and documentation
- All public items must have rustdoc documentation
- Feature flags for optional functionality (`chunks` gating `text-splitter`)
- Build system: cargo (no custom scripts)
- All tests must pass before declaring work complete
- Verify with `cargo test --all-features`

## Active Work

HTTP configuration options: see history/001-http-config-options.md. Functional controls and all local quality gates pass; see history/002-quality-gates.md for baseline cleanup and fixture portability. Cross-platform hosted verification remains pending.

## History Index

| # | Name | Status | Date |
|---|------|--------|------|
