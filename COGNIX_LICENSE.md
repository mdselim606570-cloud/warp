# Cognix v1 Licensing Boundary

## Distribution Note

This codebase is a reduced terminal-only fork of Warp. It intentionally excludes account, sync, Drive, collaboration, GraphQL, and server-backed product systems. The remaining terminal runtime is kept separate from any MIT-licensed UI framework code. Any redistribution must preserve the licensing boundary: AGPL-3.0-only product code remains under AGPL-3.0, while MIT-licensed UI framework code remains under MIT, with explicit attribution and separation from the AGPL code.

This prevents accidental mixing of license scopes and makes redistribution safe and reviewable.

## License Mapping

| Component | License | Location |
|-----------|---------|----------|
| Terminal runtime (product code) | AGPL-3.0-only | `crates/warp_terminal/`, `crates/warp_core/`, `app/src/terminal/` |
| TUI front-end | AGPL-3.0-only | `crates/warp_tui/` |
| UI framework (WarpUI) | MIT | `crates/warpui/`, `crates/warpui_core/`, `crates/warpui_extras/` |
| Core utilities | AGPL-3.0-only | `crates/warp_util/`, `crates/command/`, `crates/editor/` |
| CLI tooling | AGPL-3.0-only | `crates/warp_cli/` |
| Settings/config | AGPL-3.0-only | `crates/settings/`, `crates/settings_value/` |
| Persistence (local) | AGPL-3.0-only | `crates/persistence/` |
| AI (local execution) | AGPL-3.0-only | `crates/ai/`, `crates/ai_types/` |

## MIT-Licensed Components

The following crates are MIT-licensed and must be kept separate from AGPL product code:

- `crates/warpui/` - MIT license (see Cargo.toml)
- `crates/warpui_core/` - MIT license
- `crates/warpui_extras/` - MIT license

## AGPL-3.0-Only Components

All product code that depends on the terminal runtime is AGPL-3.0-only:

- `app/` - The main application binary
- `crates/warp_terminal/` - Terminal emulation
- `crates/warp_core/` - Core application logic
- `crates/warp_cli/` - CLI tooling
- `crates/warp_tui/` - TUI front-end
- `crates/warp_util/` - Utility functions
- `crates/editor/` - Text editing
- `crates/command/` - Command infrastructure

## Redistribution Requirements

1. Any redistribution must include both LICENSE-AGPL and LICENSE-MIT files
2. MIT-licensed UI framework code must be clearly identified as such
3. AGPL-3.0-only product code must not be combined with MIT-licensed code in a way that would subject the AGPL code to MIT terms
4. Attribution notices must be preserved from original Warp code
5. All derived works that include AGPL-3.0 code must also be AGPL-3.0 licensed
