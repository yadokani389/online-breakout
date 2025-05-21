use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy)]
pub struct Team(pub usize);
