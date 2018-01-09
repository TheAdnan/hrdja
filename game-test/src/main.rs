extern crate piston_window;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Tetrust", [640, 480])
    .opengl(opengl)
    .exit_on_esc(true)
    .build().unwrap();

    while let Some(e) = window.next(){
    	match e {
    		Input::Update(args) => {}
    		Input::Render(_) => {}
    		Input::Release(args) => {}
    		_ => {}
    	}
    } 
}
