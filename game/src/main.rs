//! RPG Game Created by Rust and Piston
//! 

#![deny(missing_docs)]

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

extern crate rpg_map;
extern crate config;
extern crate error;
extern crate player;

mod event_handler;
mod object_manager;
mod mouse_detection;

use piston::{
    window::WindowSettings,
    event_loop::{Events, EventSettings, EventLoop},
    RenderEvent,
};

use opengl_graphics::{
    OpenGL,
    GlGraphics,
    Filter,
    GlyphCache,
    TextureSettings,
};

use graphics::character::CharacterCache;

use glutin_window::GlutinWindow;

use rpg_map::Map;
use player::{CubePlayer, Player, Appearance};
use event_handler::EventHandler;
use object_manager::cubes_manager::CubesManager;

fn main() {
    println!("Welcome to the world of Rust RPG!");
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rust-rpg", [config::size::WINDOW_SIZE; 2])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Create windown failed.");

    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut gl = GlGraphics::new(opengl);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);

    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let map = Map::new(config::size::WINDOW_SIZE, config::size::MAP_BLOCK_SIZE);

    let mut cubes = Vec::new();
    for i in 0..config::size::CUBE_NUM {
        let mut name = i.to_string();
        name.insert_str(0, "cube-");
        let cube = CubePlayer::new(
            &name,
            [config::size::WINDOW_SIZE as f64 / (3 + i) as f64, config::size::WINDOW_SIZE as f64 / 2f64],
            config::color::RED,
            ('1', config::color::YELLOW)
        );
        //println!("New Cube: {:?}", cube);
        cubes.push(cube);
    }
    println!("size of cubes: {}", cubes.len());
    let mut cubes_manager = CubesManager::new(cubes);

    let mut mouse_detector = mouse_detection::MouseDetector::new();

    while let Some(e) = events.next(&mut window) {
        match cubes_manager.handle_event(&e) {
            Ok(_) => {},
            Err(err) => panic!(err),
        }
        match mouse_detector.handle_event(&e) {
            Ok(_) => {},
            Err(err) => panic!(err),
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::Transformed;
                graphics::clear(config::color::BLUE, g);

                for i in 0..map.map_size {
                    for j in 0..map.map_size {
                        let block = map.get_block(i as usize, j as usize);
                        graphics::Rectangle::new(block.color).draw(
                            [block.pos[0] as f64, block.pos[1] as f64, map.block_size as f64, map.block_size as f64],
                            &c.draw_state,
                            c.transform,
                            g,
                        )
                    }
                }

                for cube in &cubes_manager.cubes {
                    let (cube_color, cube_textture_char, cube_texture_color) = match cube.appearance() {
                        Appearance::CubeAppearance(x, y, j, k, c) => ([x, y, j, k], c.0, c.1),
                    };

                    graphics::Rectangle::new(cube_color).draw(
                        [cube.pos()[0], cube.pos()[1], config::size::MAP_BLOCK_SIZE as f64, config::size::MAP_BLOCK_SIZE as f64],
                        &c.draw_state,
                        c.transform,
                        g,
                    );

                    let cube_character = glyphs.character(32, cube_textture_char).unwrap();
                    graphics::Image::new_color(cube_texture_color).draw(
                        cube_character.texture,
                        &c.draw_state,
                        c.transform.trans(cube.pos()[0], cube.pos()[1]),
                        g,
                    )
                }
            });
        }
    }
}

