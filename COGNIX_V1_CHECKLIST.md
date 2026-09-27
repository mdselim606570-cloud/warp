# Cognix v1 - Formal Engineering Checklist

## Objective
Build a terminal-first product by removing all Warp-specific product systems that are not required for local terminal execution.

## Product Boundary
The first release must be limited to:
- Local terminal session management
- Local shell/PTY execution
- Local UI for terminal interaction
- Minimal local settings/config only if required by terminal execution
- Optional local persistence, not cloud-backed state

Everything else is out of scope for v1.

## Cut List: Do Not Port

### 1) Account and Auth
- **Action**: Delete all auth flows
- **Location**: `app/src/auth/`, `warp_server_auth`, `oauth2` dependency
- **Reason**: Not needed for local terminal use
- **Risk if kept**: Introduces identity, token, and server dependencies

### 2) Drive and Cloud Sync
- **Action**: Delete all Drive and cloud-backed storage flows
- **Location**: `app/src/drive/`, `cloud_objects`, `cloud_object_client`, `cloud_object_persistence`, `cloud_object_models`
- **Reason**: Remote state is out of v1 scope
- **Risk if kept**: Pulls in storage, sync, and object infrastructure

### 3) GraphQL Layer
- **Action**: Delete all GraphQL client/server contracts
- **Location**: `crates/graphql/`, `crates/warp_graphql_schema/`, `warp_graphql`, `cynic`, `graphql-ws-client`
- **Reason**: Not needed for local terminal execution
- **Risk if kept**: Expands schema, client, and transport stack

### 4) Server Client and Auth Bootstrap
- **Action**: Delete server auth and token plumbing
- **Location**: `warp_server_client`, `warp_server_auth`, `http_client`, `http_server`, `websocket`
- **Reason**: Not required for offline terminal use
- **Risk if kept**: Creates remote-service coupling

### 5) Team/Collaboration
- **Action**: Delete all team and collaboration features
- **Location**: `app/src/workspace/`, `app/src/workspaces/`, `session-sharing-protocol`
- **Reason**: Not part of first release
- **Risk if kept**: Adds shared-state and multi-user complexity

### 6) Cloud Object Models
- **Action**: Delete remote persistence layer
- **Location**: `crates/cloud_objects/`, `crates/cloud_object_models/`, `crates/cloud_object_persistence/`
- **Reason**: Not needed without cloud services
- **Risk if kept**: Keeps remote persistence layer alive

### 7) Billing/Subscription
- **Action**: Delete billing and subscription systems
- **Location**: `app/src/billing/`, `app/src/pricing/`
- **Reason**: No account or entitlement layer needed
- **Risk if kept**: Carries commercial/product plumbing

### 8) Warp Telemetry
- **Action**: Delete or neutralize Warp-specific telemetry
- **Location**: `crates/warp_core/src/telemetry.rs`, `app/src/server/telemetry/`, `opentelemetry` stack
- **Reason**: Product-specific instrumentation not required
- **Risk if kept**: Creates unnecessary service coupling

### 9) Onboarding and Marketing UX
- **Action**: Delete onboarding and marketing UI
- **Location**: `app/src/onboarding/`, `crates/onboarding/`, `app/src/tips/`, `app/src/banner/`
- **Reason**: Not terminal-essential
- **Risk if kept**: Adds splash, funnel, setup, and account logic

### 10) Settings UI and Account Management
- **Action**: Delete account/settings/workspace management UX
- **Location**: `app/src/settings/`, `app/src/settings_view/`, `app/src/workspaces/`
- **Reason**: Not required if only terminal execution remains
- **Risk if kept**: Carries product-specific preferences and remote options

### 11) Browser/Web Pane
- **Action**: Delete browser/web-pane features
- **Location**: `app/src/web/`, web rendering dependencies
- **Reason**: Not terminal core
- **Risk if kept**: Pulls in web rendering and remote surface layers

### 12) Notifications
- **Action**: Delete notification systems
- **Location**: `app/src/notification/`, `app/src/notebooks/` (cloud-backed)
- **Reason**: Non-terminal experience
- **Risk if kept**: Introduces product event bus and account integration

### 13) AI/Agent Mode (Cloud-Backed)
- **Action**: Remove cloud-backed AI features; keep local terminal AI execution
- **Location**: `app/src/ai/`, `app/src/ai_assistant/`, `crates/ai/`, `crates/ai_types/`
- **Reason**: Cloud AI features depend on server infrastructure
- **Risk if kept**: Creates remote service coupling

### 14) Warp UI Framework Crates
- **Action**: Exclude Warp UI framework crates from the default build unless explicitly needed
- **Location**: `crates/warpui/`, `crates/warpui_core/`, `crates/warpui_extras/`
- **Reason**: Large dependency and maintenance overhead
- **Risk if kept**: Pulls in GPU rendering, mouse input, and product-specific UI

## Keep List

The following must be retained:
- `warp_terminal` - Terminal emulation and shell/PTY execution
- `warp_core` - Core utilities (stripped of cloud features)
- `warpui_core` - TUI element library (with `tui` feature only)
- `warp_tui` - Headless TUI front-end
- `crates/editor` - Text editing functionality
- `crates/command` - Command infrastructure
- `crates/warp_cli` - CLI command parsing
- `crates/vim` - Vim keybinding support
- Terminal session management and local shell execution
- Local settings/config

## Recommended Deletion Policy

### Keep only:
- Terminal emulation
- Shell execution
- Local session state
- Minimal local config
- Terminal-first UX

### Delete:
- Anything that depends on remote identity or cloud infrastructure
- Anything that is product-specific to Warp rather than terminal functionality
- Anything that exists for collaboration, billing, onboarding, marketing, or account management

## Licensing Boundary

This codebase is a reduced terminal-only fork of Warp. It intentionally excludes account, sync, Drive, collaboration, GraphQL, and server-backed product systems. The remaining terminal runtime is kept separate from any MIT-licensed UI framework code. Any redistribution must preserve the licensing boundary: AGPL-3.0-only product code remains under AGPL-3.0, while MIT-licensed UI framework code remains under MIT, with explicit attribution and separation from the AGPL code.

## Validation Checklist

- [ ] App runs with no remote account dependency
- [ ] Local terminal features work without cloud sync
- [ ] No auth/server initialization is required at startup
- [ ] No remote collaboration or cloud object code is reachable in the default path
- [ ] Build is smaller and simpler than the original Warp product
- [ ] Cargo.toml dependency graph is simplified
- [ ] Feature flags are reduced to terminal-essential only
- [ ] All non-terminal modules are removed from app/src
- [ ] No references to auth, drive, cloud, graphql, or server in the terminal build path
