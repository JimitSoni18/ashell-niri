use std::{cell::RefCell, ops::Deref, time::Duration};

use iced_sctk::commands::layer_surface::Anchor;

use crate::config::Orientation;

pub mod audio;
pub mod battery;
pub mod brightness;
pub mod launcher;
pub mod net;
pub mod bluetooth;
pub mod powerprofiles;
pub mod privacy;
pub mod idle_inhibitor;

pub struct Commander<T> {
    sender: tokio::sync::mpsc::UnboundedSender<T>,
    receiver: RefCell<Option<tokio::sync::mpsc::UnboundedReceiver<T>>>,
}

impl<T> Commander<T> {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        Self {
            sender,
            receiver: RefCell::new(Some(receiver)),
        }
    }

    pub fn give_receiver(&self) -> Option<tokio::sync::mpsc::UnboundedReceiver<T>> {
        self.receiver.borrow_mut().take()
    }
}

impl<T> Deref for Commander<T> {
    type Target = tokio::sync::mpsc::UnboundedSender<T>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

pub enum IndicatorState {
    Normal,
    Success,
    Warning,
    Danger,
}

pub fn format_duration(duration: &Duration) -> String {
    let h = duration.as_secs() / 60 / 60;
    let m = duration.as_secs() / 60 % 60;
    if h > 0 {
        format!("{}h {:>2}m", h, m)
    } else {
        format!("{:>2}m", m)
    }
}

pub fn get_anchor(orientation: Orientation) -> Anchor {
    match orientation {
        Orientation::Horizontal => Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT),
        Orientation::Vertical => Anchor::LEFT.union(Anchor::TOP).union(Anchor::BOTTOM),
    }
}
