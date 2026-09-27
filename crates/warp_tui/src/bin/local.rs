use anyhow::Result;
use warp_core::channel::{Channel, ChannelState};
use warp_core::features;

// Cognix v1: Local terminal only. No cloud, account, or product features.
fn main() -> Result<()> {
    let config = warp_channel_config::load_config!("local");
    ChannelState::set(
        ChannelState::new(Channel::Local, config)
            .with_additional_features(features::DEBUG_FLAGS)
            .with_additional_features(features::LOCAL_FLAGS),
    );

    warp_tui::run()
}