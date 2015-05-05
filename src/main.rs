extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::input::{ Button, Key };
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    dir: Direction, // Direction for the square.
    speed: f64,
    x: f64,
    y: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let mut square = [(args.width / 2) as f64, (args.height / 2) as f64, 10.0, 10.0];
        let (x, y) = (self.x, self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            square[0] = x;
            square[1] = y;

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, c.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self.dir {
            Direction::Up    => self.y -= self.speed * args.dt,
            Direction::Down  => self.y += self.speed * args.dt,
            Direction::Right => self.x += self.speed * args.dt,
            Direction::Left  => self.x -= self.speed * args.dt
        }
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "game",
            [500, 500]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        dir: Direction::Right,
        speed: 250.0,
        x: 100.0,
        y: 100.0
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => app.dir = Direction::Up,
                Key::A => app.dir = Direction::Left,
                Key::S => app.dir = Direction::Down,
                Key::D => app.dir = Direction::Right,
                _ => ()
            }
        }
    }
}
