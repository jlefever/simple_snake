#![feature(slice_patterns)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::collections::LinkedList;

use piston::window::WindowSettings;
use piston::input::{ Button, Key };
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

type Location = [i32; 2];

enum Direction {
	Up,
	Down,
	Right,
	Left
}

struct Model {
	snake: LinkedList<Location>,
	dir: Direction
}

pub struct App {
	gl: GlGraphics, // OpenGL drawing backend.
	model: Model,
	time: f64,
	speed: f64
}

impl Model {
	fn tick(&mut self) {
		let &[x, y] = self.snake.front().unwrap();
		match self.dir {
			Direction::Up    => self.snake.push_front([x, y - 1]),
			Direction::Down  => self.snake.push_front([x, y + 1]),
			Direction::Right => self.snake.push_front([x + 1, y]),
			Direction::Left  => self.snake.push_front([x - 1, y])
		}
		self.snake.pop_back();
	}
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		const SIZE: f64 = 5.0;

		let ref snake = self.model.snake;

		self.gl.draw(args.viewport(), |c, gl| {
			// Clear the screen.
			clear(BLACK, gl);

			for loc in snake {
				rectangle(WHITE, [loc[0] as f64 * SIZE, loc[1] as f64 * SIZE, SIZE, SIZE], c.transform, gl);
			}

		});
	}

	fn update(&mut self, args: &UpdateArgs) {
		self.time += self.speed * args.dt;
		if self.time >= 1.0 {
			self.time = 0.0;
			self.model.tick();
		}
	}
}

fn main() {
	let opengl = OpenGL::_3_2;

	// Create an Glutin window.
	let window = Window::new(
		opengl,
		WindowSettings::new(
			"Simple Snake",
			[1600, 900]
		)
		.exit_on_esc(true)
	);

	let mut snake = LinkedList::new();
	snake.push_front([5, 7]);
	snake.push_front([5, 8]);
	snake.push_front([5, 9]);
	snake.push_front([6, 9]);

	let model = Model {
		snake: snake,
		dir: Direction::Right
	};

	let mut app = App {
		gl: GlGraphics::new(opengl),
		model: model,
		time: 0.0,
		speed: 25.0
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
				Key::W => app.model.dir = Direction::Up,
				Key::A => app.model.dir = Direction::Left,
				Key::S => app.model.dir = Direction::Down,
				Key::D => app.model.dir = Direction::Right,
				_ => ()
			}
		}
	}
}
