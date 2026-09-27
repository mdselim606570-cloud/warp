//! Preview-channel `warp-tui` binary.
//!
//! Cognix v1: Local terminal only. No cloud or product features.

use anyhow::Result;
use warp_core::channel::{Channel, ChannelState};

fn main() -> Result<()> {
    ChannelState::set(ChannelState::new(
        Channel::Preview,
        warp_channel_config::load_config!("preview"),
    ));

    warp_tui::run()
}
