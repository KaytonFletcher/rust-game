use amethyst::{
    assets::{ProgressCounter},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera },
    window::ScreenDimensions,
};

use crate::{entities::init_player, resources::asset};

#[derive(Default)]
pub struct LoadState {
    progress_counter: Option<ProgressCounter>,
}

impl SimpleState for LoadState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Load our sprites and display them
        //let sprites = load_sprites(world);
        // init_sprites(world, &sprites, &dimensions);

        self.progress_counter = Some(asset::load_assets(world, vec![asset::AssetType::Player]));

        // Place the camera
        init_camera(world, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                println!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }



    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
              

                let player_prefab_handle = {
                    let prefab_list = data.world.read_resource::<asset::PrefabList>();
                    prefab_list.get(asset::AssetType::Player).unwrap().clone()
                };
                
                let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

                init_player(data.world, player_prefab_handle, &dimensions);

                self.progress_counter = None;
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

// fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
//     // Load the texture for our sprites. We'll later need to
//     // add a handle to this texture to our `SpriteRender`s, so
//     // we need to keep a reference to it.
//     let texture_handle = {
//         let loader = world.read_resource::<Loader>();
//         let texture_storage = world.read_resource::<AssetStorage<Texture>>();
//         loader.load(
//             "sprites/knight/knight.png",
//             ImageFormat::default(),
//             (),
//             &texture_storage,
//         )
//     };

//     // Load the spritesheet definition file, which contains metadata on our
//     // spritesheet texture.
//     let sheet_handle = {
//         let loader = world.read_resource::<Loader>();
//         let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
//         loader.load(
//             "sprites/knight/knight.ron",
//             SpriteSheetFormat(texture_handle),
//             (),
//             &sheet_storage,
//         )
//     };

//     // Create our sprite renders. Each will have a handle to the texture
//     // that it renders from. The handle is safe to clone, since it just
//     // references the asset.
//     (0..3)
//         .map(|i| SpriteRender {
//             sprite_sheet: sheet_handle.clone(),
//             sprite_number: i,
//         })
//         .collect()
// }
