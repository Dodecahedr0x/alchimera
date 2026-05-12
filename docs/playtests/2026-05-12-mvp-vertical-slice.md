# MVP Vertical Slice Manual Playtest — 2026-05-12

## Scope

- **Project:** Alchimera MVP vertical slice
- **Branch/worktree:** `docs/mvp-playtest-checklist` at `/home/openclaw/worktrees/alchimera/w5-t2-playtest`
- **Environment:** Headless scheduled cron job on Linux; no human operator, display, mouse/keyboard capture, or GPU-backed visual session was available.
- **Seed planned for manual run:** `mvp-vertical-slice-2026-05-12` (or the nearest supported equivalent once seed entry is exposed in the runtime UI/CLI).

Because this run executed in a headless cron environment, automated boot/test evidence was recorded, and checks requiring visual confirmation or real-time manual input are marked **Pending manual visual run** rather than passed.

## Automated evidence collected

| Check | Result | Evidence / notes |
| --- | --- | --- |
| Binary help path | Pass | `cargo run -- --help` completed successfully and printed the Alchimera usage text. |
| Runtime boot smoke | Pass with timeout harness | `timeout 5s cargo run` compiled/started `target/debug/alchimera`; process remained in the Bevy run loop until the timeout terminated it with exit code `124`. No panic or startup error was printed before timeout. |
| Workspace regression tests | Pass | `cargo test --workspace` completed successfully. |

## Manual MVP checklist

| # | Checklist item | Status | Notes |
| --- | --- | --- | --- |
| 1 | Boot game | **Automated smoke pass / manual visual pending** | Runtime starts under `cargo run`; visual window/session not available in cron. |
| 2 | Start world with seed | **Pending manual visual run** | Use seed `mvp-vertical-slice-2026-05-12` or nearest supported seed input. Headless run could not verify seed selection UX. |
| 3 | Walk on terrain | **Pending manual visual run** | Requires live input and viewport observation. |
| 4 | Observe terrain chunk loading | **Pending manual visual run** | Automated tests cover chunk request behavior; visual streaming/loading must be confirmed in an interactive session. |
| 5 | See trees/rocks | **Pending manual visual run** | Generation/object tests pass, but visual presence in the MVP scene needs manual confirmation. |
| 6 | Target object | **Pending manual visual run** | Interaction raycast tests pass; in-game targeting reticle/feedback needs visual confirmation. |
| 7 | Harvest wood/stone | **Pending manual visual run** | Harvesting tests pass for yield and modification-log behavior; manual input/feedback not verified. |
| 8 | Open inventory/hotbar | **Pending manual visual run** | Inventory/hotbar tests pass for resource and shell behavior; visual UI needs manual confirmation. |
| 9 | Craft stone axe or placeholder recipe | **Pending manual visual run** | Crafting tests pass; manual recipe availability and UX need verification. If no stone axe recipe is exposed yet, verify the placeholder recipe instead. |
| 10 | Place workbench/beam/wall | **Pending manual visual run** | Building placement tests pass; manual placement preview/final object visuals need verification. |
| 11 | Save, quit, reload | **Pending manual visual run** | Persistence tests pass; manual save/load flow and controls need verification. |
| 12 | Confirm inventory and world modifications persist | **Pending manual visual run** | Automated persistence tests pass for inventory, placed objects, and harvested-object overrides. End-to-end runtime reload still needs manual confirmation. |
| 13 | Record FPS/frame-time notes | **Pending manual visual run** | No GPU/display session was available. Use diagnostics overlay or external frame capture in a real playtest. |

## FPS / frame-time notes

- **This run:** Not measured. Headless cron cannot provide representative rendered FPS or frame-time data.
- **Manual run instructions:** Enable the diagnostics overlay if available, stand still after initial chunk load, then record approximate FPS/frame time at:
  1. Initial spawn after world start.
  2. While walking across at least one chunk boundary.
  3. While looking at trees/rocks and interacting/harvesting.
  4. After placing a build object and reloading the save.

## Recommended manual session script

1. Launch the game from the project root with `cargo run` or `cargo run --release`.
2. Start a world using seed `mvp-vertical-slice-2026-05-12` if the current UI/CLI supports explicit seeds.
3. Walk forward/sideways over varied terrain for at least 60 seconds.
4. Watch for terrain chunk pop-in, stalls, missing collision, or visible seams.
5. Locate trees and rocks; target each object and note any targeting UI/feedback.
6. Harvest wood and stone; confirm inventory/hotbar counts update.
7. Craft the stone axe, or a placeholder recipe if the stone axe is not exposed.
8. Place a workbench, beam, or wall; note placement preview accuracy and final object stability.
9. Save, quit, reload the same world, and verify inventory plus harvested/placed world modifications persist.
10. Record FPS/frame-time observations and any blocking bugs in this document or a follow-up issue.

## Follow-up limitations

- Manual visual/input validation was not run because this task executed unattended in a headless cron environment.
- The runtime boot smoke intentionally used a timeout because the Bevy app enters its run loop and does not exit on its own.
- Performance numbers here are intentionally omitted rather than estimated; W5-T3 should capture a representative baseline in an environment capable of rendering the game.
