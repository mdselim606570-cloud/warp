# Cognix v1 - Change Summary

## Overview
This document tracks all changes made to transform Warp into Cognix v1, a terminal-only product stripped of all cloud, account, and product features.

## Phase 1: Documentation & Configuration

### Files Created
- `COGNIX_V1_CHECKLIST.md` - Formal engineering checklist
- `COGNIX_LICENSE.md` - Licensing boundary documentation (AGPL-3.0-only product / MIT UI framework)
- `COGNIX_V1_SUMMARY.md` - This file

### Files Modified
- `AGENTS.md` - Added Cognix v1 guidance section
- `Cargo.toml` (workspace) - Stripped from 100+ crates to ~20 terminal-essential crates
- `app/Cargo.toml` - Stripped ~700 product feature flags → ~15 terminal-only features
- `crates/warp_core/Cargo.toml` - Removed `settings`, `websocket`, `command-corrections`, `warpui_extras`
- `crates/warp_terminal/Cargo.toml` - Removed `warp_assets`, `warp_isolation_platform`, `secret_redaction`, `session-sharing-protocol`, `regex_dfas`
- `crates/warp_tui/Cargo.toml` - Removed `ai`, `http_client`, `warp_channel_config`, `warp_editor`, `warp_files`, `warp_search_core`, `settings_value`
- `crates/warpui/Cargo.toml` - Removed `settings_value` feature
- `crates/warpui_core/Cargo.toml` - Removed `settings_value` feature
- `crates/warp_cli/Cargo.toml` - Removed `command-corrections`
- `app/src/lib.rs` - Stripped ~96 module declarations and imports
- All `bin/*` entry points (`local.rs`, `oss.rs`, `dev.rs`, `preview.rs`, `stable.rs`) - Updated to local-terminal-only config
- `crates/warp_features/src/lib.rs` - Removed `DOGFOOD_FLAGS`, `PREVIEW_FLAGS`

### Workspace Cargo.toml Changes
- Removed `lsp-types`, `get-size`, `rayon` from workspace dependencies
- Rewrote `[workspace.dependencies]` to only include terminal-essential crates
- Updated `default-members` to only include terminal crates

## Phase 2: Concrete Deletions (2763 files, ~955,000 lines)

### Deleted Crate Directories (30+)
- `cloud_objects`, `cloud_object_client`, `cloud_object_models`, `cloud_object_persistence`
- `cloud_object_client`, `graphql`, `warp_graphql_schema`
- `warp_server_auth`, `warp_server_client`, `remote_server`, `http_server`, `websocket`
- `ai`, `ai_types`, `warp_harness_usage`
- `onboarding`, `computer_use`, `node_runtime`
- `persistence`, `warp_files`, `firebase`, `managed_secrets`, `managed_secrets_wasm`
- `lsp`, `mcp`, `natural_language_detection`, `prevent_sleep`
- `regex_dfas`, `repo_metadata`, `secret_redaction`, `serve-wasm`
- `settings`, `settings_value`, `settings_value_derive`, `simple_logger`
- `string-offset`, `syntax_tree`, `ui_components`, `virtual_fs`, `voice_input`
- `watcher`, `integration`, `ipc`, `ipynb_parser`, `jsonrpc`
- `languages`, `local_control`, `warp_isolation_platform`, `field_mask`
- `fuzzy_match`, `http_client`, `input_classifier`, `app-installation-detection`
- `build_cache`, `warp_command_signatures`

### Deleted App Source Directories (40+)
- `app/src/ai/`, `app/src/auth/`, `app/src/billing/`, `app/src/cloud_object/`
- `app/src/context_chips/`, `app/src/drive/`, `app/src/onboarding/`
- `app/src/server/`, `app/src/workspaces/`, `app/src/workspace/`
- `app/src/voice/`, `app/src/notification/`, `app/src/tips/`
- `app/src/ai_assistant/`, `app/src/code_review/`, `app/src/notebooks/`
- `app/src/wasm_nux_dialog.rs`, `app/src/window_settings.rs`, `app/src/voltron.rs`
- `app/src/settings_view/`, `app/src/tab_configs/`, `app/src/themes/`
- `app/src/search/`, `app/src/usage/`, `app/src/uri/`
- `app/src/integration_testing/`, `app/src/coding_entrypoints/`
- `app/src/launch_configs/`, `app/src/autoupdate/`, `app/src/crash_reporting/`
- `app/src/banner/`, `app/src/suggestions/`, `app/src/system/`
- `app/src/tracing/`, `app/src/ui_components/`, `app/src/view_components/`
- `app/src/undo_close/`, `app/src/buy_credits_banner.rs`

### Remaining App Source Structure
- `app/src/alloc.rs`, `app_state.rs`, `app_state_tests.rs`
- `app/src/appearance.rs`, `app/src/bin/`, `app/src/channel.rs`
- `app/src/code/`, `app/src/command_palette.rs`, `app/src/completer/`
- `app/src/default_terminal/`, `app/src/editor/`, `app/src/env_vars/`
- `app/src/features.rs`, `app/src/input_classifier.rs`
- `app/src/interval_timer.rs`, `app/src/keyboard.rs`
- `app/src/local_control/`, `app/src/menu.rs`, `app/src/modal.rs`
- `app/src/platform/`, `app/src/prefix.rs`, `app/src/pane_group/`
- `app/src/persistence/`, `app/src/platform/`
- `app/src/root_view.rs`, `app/src/settings/`, `app/src/shell_indicator.rs`
- `app/src/system.rs`, `app/src/tab.rs`, `app/src/terminal/`
- `app/src/tui/`, `app/src/tui_export/`, `app/src/ui_components/`
- `app/src/undo_close.rs`, `app/src/util/`, `app/src/vim_registers.rs`
- `app/src/window_settings.rs`, `app/src/word_block_editor.rs`

## Remaining Work

### P0 - Still Pending (57 files)
These files still contain references to deleted crates and need further stripping:
- `command_corrections` references in `warp_core/src/command.rs`, `warp_terminal/src/shell/mod.rs`, `warp_features/src/lib.rs`, and several app files
- `warp_files` references in `app/src/code/`, `app/src/pane_group/`, `app/src/terminal/`
- `warp_assets` references in `app/src/terminal/`, `app/src/appearance.rs`
- `remote_server` references in `app/src/terminal/`, `app/src/util/`
- `crash_recovery` references in `app/src/lib.rs`
- `workflows` references in `app/src/terminal/input/`, `app/src/command_palette.rs`

### P1 - Settings Module
The `settings` module exists in `app/src/settings/` and is used by ~220 files. These references are valid (local module, not deleted crate) but may need simplification.

### P2 - UI Framework
`warpui`, `warpui_core`, `warpui_extras` were retained for terminal rendering but may need feature flag simplification.

## Licensing
- AGPL-3.0-only: Terminal runtime, product code (`app/`, `crates/warp_core/`, `crates/warp_terminal/`, `crates/warp_tui/`, `crates/warp_cli/`)
- MIT: UI framework code (`crates/warpui/`, `crates/warpui_core/`, `crates/warpui_extras/`)

## Build Configuration
- `cargo run` / `./script/run` - Build and run the terminal app
- `./script/run-tui` - Build and run the headless TUI front-end
- `cargo check -p app --no-default-features --features "tui"` - Validate the reduced build
