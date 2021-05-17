use crate::world::World;

pub const DEFAULT_WORLD: World = World {
    name: "World",
    aliases: &[],
    children: &[],
    r#move: None,
};
