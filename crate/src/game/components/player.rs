use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
	type Storage = specs::NullStorage<Self>;
}