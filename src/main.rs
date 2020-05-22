mod utils;
mod camera;
mod map;

use amethyst::{
    prelude::*,
    core::transform::{Transform, TransformBundle},
    renderer::{
        SpriteRender, SpriteSheet,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    ecs::prelude::{Component, DenseVecStorage},
    input::{InputBundle, StringBindings},
    assets::{Handle, ProgressCounter},
    ui::{RenderUi, UiBundle, UiCreator},
};
use amethyst_tiles::{MortonEncoder, RenderTiles2D};
use utils::load_sprite_sheet;
use camera::{initialise_camera, CameraSystem, MouseRaycastSystem};
use map::{initialise_map, BlockTile};


pub const BLOCK_HEIGHT: f32 = 16.0;
pub const BLOCK_WIDTH: f32 = 4.0;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const BLOCK_SIZE: i32 = 32;


pub struct Block {
    pub width: f32,
    pub height: f32,
}

impl Block {
    fn new() -> Block {
        Block {
            width: BLOCK_WIDTH,
            height: BLOCK_HEIGHT,
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_blocks(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, sprite_number: usize) {
    let mut block_transform = Transform::default();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };
    
    block_transform.set_translation_xyz(BLOCK_WIDTH * 0.5, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Block::new())
        .with(block_transform)
        .build();
}

#[derive(Default)]
struct GameState {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.progress_counter = Some(Default::default());

        world.register::<Block>();
        
        let sprite_sheet_handle = load_sprite_sheet(
            world, "images/hyptosis_sprites.png", "images/hyptosis_sprites.ron"
        );

        initialise_blocks(world, sprite_sheet_handle, 0);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create(
                "ui/main.ron",
                self.progress_counter.as_mut().unwrap(),
            );
        });
        
        initialise_camera(world);

        let batch_1_sprite_sheet_handle = load_sprite_sheet(
            world, "images/hyptosis_tile-art-batch-1.png", "images/hyptosis_tile-art-batch-1.ron"
        );
        initialise_map(world, batch_1_sprite_sheet_handle);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<BlockTile, MortonEncoder>::default()),
        )?
        .with_bundle(TransformBundle::new())?

        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?

        .with(CameraSystem, "camera_system", &["input_system"])
        .with(MouseRaycastSystem, "mouse_raycast_system", &["input_system"])
        ;

    let mut game = Application::new(assets_dir, GameState::default(), game_data)?;
    game.run();

    Ok(())
}
