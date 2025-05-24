use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy)]
pub struct Team(pub usize);

impl Team {
    pub const ITEM: Self = Self(2);

    pub fn hue(&self) -> f32 {
        match self.0 {
            0 => 0.,
            1 => 180.,
            _ => 60.,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct Count(pub usize);
