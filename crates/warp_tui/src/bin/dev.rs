//! Dev-channel `warp-tui` binary (internal nightly builds).
//!
//! Cognix v1: Local terminal only. No cloud or product features.

use anyhow::Result;
use warp_core::channel::{Channel, ChannelState};
use warp_core::features;

fn main() -> Result<()> {
    ChannelState::set(
        ChannelState::new(Channel::Dev, warp_channel_config::load_config!("dev"))
            .with_additional_features(features::DEBUG_FLAGS)
            .with_additional_features(features::LOCAL_FLAGS),
    );

    warp_tui::run()
}
