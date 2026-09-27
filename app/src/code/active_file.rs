//! Module containing the definition of [`Active`],
//! which tracks the currently focused file across an entire PaneGroup.

use warpui::{Entity, ModelContext};

use super::buffer_location::LocalOrRemotePath;

/// Events emitted by the Active.
#[derive(Debug, Clone)]
pub enum ActiveFileEvent {
    /// A new file became focused.
    ActiveFileChanged { location: LocalOrRemotePath },
}

/// Model that tracks the currently focused file.
#[derive(Default)]
pub struct Active {
    /// The currently focused file, if any.
    active_file: Option<LocalOrRemotePath>,
}

impl Entity for Active {
    type Event = ActiveFileEvent;
}

impl Active {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the currently active file, if any.
    pub fn active_file(&self) -> Option<&LocalOrRemotePath> {
        self.active_file.as_ref()
    }

    /// Set the currently active file.
    pub fn active_file_changed(
        &mut self,
        location: LocalOrRemotePath,
        ctx: &mut ModelContext<Self>,
    ) {
        // Only emit event if the active file changed.
        if self.active_file.as_ref() != Some(&location) {
            self.active_file = Some(location.clone());
            ctx.emit(ActiveFileEvent::ActiveFileChanged { location });
            ctx.notify();
        }
    }
}
