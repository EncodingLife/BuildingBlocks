use bevy::prelude::*;

use crate::mods::{cell::stem::Stem, elements::basic::Element};

#[derive(Component)]
pub struct Makeup([Element;4]);

pub trait HasMakeup {
    fn get_makeup() -> Makeup;
}

impl HasMakeup for Stem {
    fn get_makeup() -> Makeup {
        Makeup([Element::Mass, Element::Act, Element::None, Element:: None])
    }
}