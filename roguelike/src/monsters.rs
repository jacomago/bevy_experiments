const MONSTER_SPRITE_INDEX: usize = 1;

#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    _m: Monster,
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

fn setup(rooms: Vec<Rect>) {}
