use anyhow::Result;
use warp_core::channel::{Channel, ChannelConfig, ChannelState};

// Cognix v1: Local terminal only. No cloud, account, or product features.
fn main() -> Result<()> {
    let config = warp_channel_config::load_config!("local");
    let mut state = ChannelState::new(Channel::Local, config);
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);

    warp_tui::run()
}