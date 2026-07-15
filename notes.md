# Repo Assist Memory — iamnbutler/motif

## Last Updated
2026-07-15

## Repository State
- Main branch: `57a5ad8` (feat(ci): add release workflow and version bump helper)
- No tags/releases exist (version: 0.0.1)
- 57 open Repo Assist draft PRs (all from github-actions[bot])
- 6 open issues (all have Repo Assist comments)
- No human-authored PRs or recent human activity

## Open Issues
- #15: [agentics] Repo Assist failed — labeled `wontfix`, expired Mar 2026 → suggest close
- #48: [Repo Assist] PR backlog merge order — 17 draft PRs guide (outdated: now 57 PRs)
- #49: [Repo Assist] Research: wgpu rendering backend
- #51: [Repo Assist] Research: text metrics debug example
- #94: [Repo Assist] Research: view memoization (Option C ready: ~40 lines, no PartialEq on Quad/TextRun/GlyphInstance)
- #95: [Repo Assist] Monthly Activity 2026-07 (current month summary)

## Comments Made
All 6 open issues have at least one Repo Assist comment. Do not re-engage unless new human activity appears.

## Monthly Summary Issue
- Current: #95 (July 2026)
- Updated: 2026-07-14

## Backlog Cursor
- Issue backlog cursor: all issues covered; reset when new issues appear
- PR cursor: all 57 PRs from Repo Assist, no human PRs to nudge

## Round-Robin Task History
- 2026-07-15: Tasks 7, 8, 10, 11
- 2026-07-14: Tasks 5, 6, 9, 10, 11
- 2026-07-13: Tasks 5, 9, 10, 11
- 2026-07-12: Tasks 2, 3, 11
- 2026-07-11: Tasks 1, 4 (deps check), 6, 7, 11
- 2026-07-10: Tasks 1, 8 (release check), 7, 6, 11
- 2026-07-09: Tasks 1, 4, 7, 11
- 2026-07-08: Tasks 7, 9, 6, 4, 11
- 2026-07-07: Task 4 (deps update PR #92), 11
- 2026-07-06: Tasks 1 (issue review), 7 (labels), 11
- 2026-07-05: Task 3 (codebase scan/view memoization), 1, 9, 11
- 2026-07-04: Tasks 1, 9, 11
- 2026-07-03: Tasks 3 (TODO audit), 7, 11
- 2026-07-02: Task 5 (PR count correction + mergeability), 1, 9, 11
- 2026-07-01: Task 11 (new monthly summary)

## Priority PRs (conflict-free, ready to merge)
- #36: Multi-click detection (p1 blocker)
- #58: Per-item inline editing TodoMVC (p1)
- #71: Element tree query devtools (p1)
- #39: max_width text wrapping fix (TODO fix)
- #53: Clipboard TODOs fix (TODO fix)
- #92: Dependency updates (refreshed 2026-07-07)
- #41: opt-level=3 for deps

## PRs to Close
- #20: Superseded by maintainer PR #35
- #21: Superseded by #71

## PR Notes
- GitHub blocks CI triggers for github-actions[bot] PRs via GITHUB_TOKEN — maintainer must manually mark PRs as "Ready for review" to run CI
- All 57 PRs from Repo Assist; no human PRs exist

## Dependency Audit
- Last audit: 2026-07-07 (PR #92 updated with full dep refresh)
- Next audit: 2026-08-07
- Post-PR-#92 merges: plan glam v0.33.x, rand v0.10.x follow-up

## Fix Attempts
- PR #92: Dependency updates (refreshed 2026-07-07, awaiting merge)
- All p1/p2 TODO items have open PRs

## Improvement Ideas Submitted (do not re-propose)
- View memoization research (issue #94)
- wgpu backend research (issue #49)
- Text metrics debug example (issue #51, PR #56)
- Theme trait (PR #44)
- PR backlog merge order guide (issue #48)

## Structural Notes
- Project: immediate-mode Rust UI framework, Metal GPU backend, macOS only currently
- Codebase: motif, motif_core, motif_debug, motif_debug_cli, motif_test crates
- Uses Taffy for layout, swash for text rasterization
- Active streams: text-input-rewrite, todomvc, layout, theming, cross-platform, devtools, animation, accessibility
- All major feature areas have open Repo Assist PRs
