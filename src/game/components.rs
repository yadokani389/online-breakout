use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, PartialEq, Clone, Copy)]
pub struct Team(pub usize);
