use ggez;
use rand;

use ggez::{graphics, Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler, MouseButton, KeyCode,KeyMods};
use ggez::input::mouse;
use ggez::input::mouse::button_pressed;
use std;
use std::collections::LinkedList;

mod world;
mod towers;

use towers::Tower;
use world::GridPosition;
use crate::towers::ArrowTower;

const GRID_SIZE: (i16, i16) = (50, 50);

const GRID_CELL_SIZE: (i16, i16) = (15, 15);

const MENU_SIZE :i16 = 100;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    (GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32) + MENU_SIZE as f32,
);


struct MyDefence {
    towers: LinkedList<ArrowTower>,
}

impl MyDefence {
    pub fn new(_ctx: &mut Context ) -> MyDefence {
        // Load/create resources such as images here.
        let mut towers = LinkedList::new();

        MyDefence {
            towers
        }
    }
}

impl EventHandler for MyDefence {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        button_pressed(_ctx, mouse::MouseButton::Left);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.2, 0.3, 1.0));

        for tower in self.towers.iter(){
            tower.draw( ctx );
        }

        // Draw code here...
        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {

    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {

        if _y <=  GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32{

            let pos = GridPosition::from((_x,_y));

            self.towers.push_back(ArrowTower::new(pos));

            println!("{} {}", pos.x,pos.y);

        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {

    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {


    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {

    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {

    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {

    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {

    }
}

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_defence", "Michael Rain")
        .window_setup(ggez::conf::WindowSetup::default().title("My Defence"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut my_game = MyDefence::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}