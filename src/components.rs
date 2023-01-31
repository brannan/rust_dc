pub use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
