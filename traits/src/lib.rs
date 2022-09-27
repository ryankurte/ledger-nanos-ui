#![no_std]

use core::fmt::Debug;

pub trait Element {
    /// Handle an event, updating element state or exiting
    fn evt(&mut self, evt: Event);

    /// Draw element
    fn draw(&self);
}

pub trait Menu {
    type State: Debug;

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Event {
    Left,
    Right,
    Both,
}