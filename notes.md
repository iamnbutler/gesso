# Repo Assist Memory — iamnbutler/motif

## Last Updated
2026-08-24

## Repository State
- Main branch: `57a5ad8` (feat(ci): add release workflow and version bump helper) — unchanged since March 2026
- No tags/releases exist (version: 0.0.1)
- 63 open Repo Assist draft PRs
- 13 open issues (all have Repo Assist comments)
- No human-authored PRs or recent human activity

## Open Issues
- #15: [agentics] Repo Assist failed — labeled `wontfix`, expired Mar 2026 → suggest close
- #48: [Repo Assist] PR backlog merge order — updated 2026-08-24, now covers all 63 PRs
- #49: [Repo Assist] Research: wgpu rendering backend
- #51: [Repo Assist] Research: text metrics debug example
- #94: [Repo Assist] Research: view memoization (implemented as PR #97)
- #98: [Repo Assist] docs(claude.md) issue artifact — suggest close (resolved by PR #99)
- #101–#107: [Repo Assist] ci(bench) push-failed issues — blocked, maintainer must apply manually
- #103: [Repo Assist] Monthly Activity 2026-08 (current month summary)

## Comments Made
All 13 open issues have at least one Repo Assist comment. Do not re-engage unless new human activity appears.

## Monthly Summary Issue
- Current: #103 (August 2026)
- Updated: 2026-08-24

## Backlog Cursor
- Issue backlog cursor: all issues covered; reset when new issues appear
- PR cursor: all 63 PRs from Repo Assist, no human PRs to nudge

## Round-Robin Task History
- 2026-08-24: Tasks 5, 10, 11
- 2026-08-23: Tasks 7, 10, 11
- 2026-08-22: Tasks 1, 2, 6, 9, 11
- 2026-08-21: Tasks 3, 11
- 2026-08-20: Tasks 1, 5, 11
- 2026-08-19: Tasks 1, 5, 8, 11
- 2026-08-18: Tasks 1, 7, 11
- 2026-08-17: Tasks 1, 4, 5, 11

## Priority PRs (conflict-free, ready to merge)
- #36: Multi-click detection (p1 blocker)
- #58: Per-item inline editing TodoMVC (p1)
- #71: Element tree query devtools (p1)
- #39: max_width text wrapping fix (TODO fix)
- #53: Clipboard TODOs fix (TODO fix)
- #92: Dependency updates (refreshed 2026-08-02 with 162 updates)
- #41: opt-level=3 for deps
- #97: View memoization

## PRs to Close
- #20: Superseded by maintainer PR #35
- #21: Superseded by #71

## PR Notes
- GitHub blocks CI triggers for github-actions[bot] PRs via GITHUB_TOKEN — maintainer must manually mark PRs as "Ready for review" to run CI
- All 63 PRs from Repo Assist; no human PRs exist
- Full merge order guide in issue #48 (last updated 2026-08-24)

## Dependency Audit
- Last audit: 2026-08-02 (PR #92 updated with 162 package updates)
- Next audit: 2026-09-07

## TODO Coverage
- All p1/p2 TODO items: open PRs exist
- P3 "Visual regression test harness": PR #109 (done 2026-08-23)
- P3 "Move hardcoded colors to theme": blocked on PR #44 (Theme trait) merging first

## Bench CI Fix
- Status: BLOCKED — modifying .github/workflows/ requires workflow permission scope
- Maintainer must apply patch from issue #107 or run 31635670794 artifacts
- Then close issues #101, #102, #104, #105, #106, #107

## Structural Notes
- Project: immediate-mode Rust UI framework, Metal GPU backend, macOS only currently
- Codebase: motif, motif_core, motif_debug, motif_debug_cli, motif_test crates
- Uses Taffy for layout, swash for text rasterization
- Active streams: text-input-rewrite, todomvc, layout, theming, cross-platform, devtools, animation, accessibility
- All major feature areas have open Repo Assist PRs
