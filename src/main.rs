#![feature(globs)]

extern crate shader_version;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_window::Sdl2Window;
use event::{ Events, WindowSettings };

fn main() {
    let opengl = shader_version::OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Image".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let image = Path::new("./bin/assets/rust-logo.png");
    let image = Texture::from_path(&image).unwrap();
    let ref mut gl = Gl::new(opengl);
    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::RenderEvent;

        e.render(|args| {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, ..4], gl);
                graphics::image(&image, &c, gl);
            });
        });
    }
}

