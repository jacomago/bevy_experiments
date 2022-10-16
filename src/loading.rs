use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

/// Plugin to load assets into the game
pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<TextureAtlasAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

/// Font assets
#[derive(AssetCollection)]
pub struct FontAssets {
    /// Fira Sans font
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

/// Audio assets
#[derive(AssetCollection)]
pub struct AudioAssets {
    /// A flying sound
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

/// Textures
#[derive(AssetCollection)]
pub struct TextureAssets {
    /// Logo asset
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

/// Texture maps
#[derive(AssetCollection)]
pub struct TextureAtlasAssets {
    /// Map of sprites
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 16, rows = 16,))]
    #[asset(path = "textures/dungeonfont.png")]
    pub texture_atlas: Handle<TextureAtlas>,
}
