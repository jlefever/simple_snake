#![feature(slice_patterns)]
extern crate rand;
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

// Location represented as array of two i32's (x and y) in in-game units
type Location = [i32; 2];

#[derive(PartialEq)]
enum Direction {
	Up,
	Down,
	Right,
	Left
}

// The game is split into the problem domain (Model) and the presentation (View).
// Although this resembles MVC, there is no definitive controller and for an application
// this small, one is not needed.
struct Model {
	snake: LinkedList<Location>, // snake, represented as a linked list of locations
	food: Location,   // location of food
	dir: Direction,   // direction snake is going
	game_width: i32,  // width of game in in-game units
	game_height: i32, // height of game in in-game units
	gameover: bool    // flag for game over
}

pub struct View {
	gl: GlGraphics, // OpenGL drawing backend.
	model: Model,   // the view's instance of the model
	unit_size: i32, // size in pixels of one in-game unit
	timer: f64,     // used in update method to keep track of time
	speed: f64			// game speed (ex: 20.0)
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

		// Check for collision with food
		if x == self.food[0] && y == self.food[1] {
			self.randomize_food(); // randomize food and avoid popping the back off (therefor growing 1 unit)
		} else {
			self.snake.pop_back();
		}

		// Check for collision with wall
		if x >= self.game_width || x < 0 || y >= self.game_height || y < 0 {
			self.gameover = true;
		}

		// Check for collision with itself
		let mut count = 0;
		for loc in &self.snake {
			if x == loc[0] && y == loc[1] {
				count += 1;
			}
			if count > 1 {
				self.gameover = true;
			}
		}
	}

	fn randomize_food(&mut self) {
		use rand::Rng;

		let mut rng = rand::thread_rng();
		println!("{}", rng.gen::<f64>());

		let x: i32 = (rng.gen::<f64>() * self.game_width as f64) as i32;
		let y: i32 = (rng.gen::<f64>() * self.game_height as f64) as i32;

		self.food = [x, y];
	}
}

impl View {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		// rebind view elements to local variables for use in self.gl.draw()
		let ref snake = self.model.snake;
		let ref food = self.model.food;
		let gameover = self.model.gameover;
		let size = self.unit_size as f64;

		self.gl.draw(args.viewport(), |c, gl| {
			if gameover {
				// simply white out the screen if there is a game over
				clear(WHITE, gl);
			} else {
				// Clear the screen
				clear(BLACK, gl);

				// Draw snake
				for loc in snake {
				rectangle(WHITE, [loc[0] as f64 * size, loc[1] as f64 * size, size, size], c.transform, gl);

				// Draw food
				rectangle(WHITE, [food[0] as f64 * size, food[1] as f64 * size, size, size], c.transform, gl);
			}
		}
	});
	}

	fn update(&mut self, args: &UpdateArgs) {
		self.timer += self.speed * args.dt;
		if self.timer >= 1.0 {
			self.timer = 0.0;
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
			[600, 450]
		)
		.exit_on_esc(true)
	);

	// Create a snake (a linked list) with an inital size of 4
	let mut snake = LinkedList::new();
	snake.push_front([5, 7]);
	snake.push_front([5, 8]);
	snake.push_front([5, 9]);
	snake.push_front([6, 9]);

	// Create a model
	let model = Model {
		snake: snake,
		food: [20, 20],
		dir: Direction::Right,
		game_width: 120,
		game_height: 90,
		gameover: false
	};

	// Create a view
	let mut view = View {
		gl: GlGraphics::new(opengl),
		model: model,
		unit_size: 5,
		timer: 0.0,
		speed: 25.0
	};

	// Main game loop
	for e in window.events() {
		if let Some(r) = e.render_args() {
			view.render(&r);
		}

		if let Some(u) = e.update_args() {
			view.update(&u);
		}

		// WASD controls
		// Does not allow a 180 turn; for instanse, you cannot immediately start going up if you were going down
		if let Some(Button::Keyboard(key)) = e.press_args() {
			view.model.dir = match key {
				Key::W => match view.model.dir == Direction::Down  { true => view.model.dir, false => Direction::Up    },
				Key::A => match view.model.dir == Direction::Right { true => view.model.dir, false => Direction::Left  },
				Key::S => match view.model.dir == Direction::Up    { true => view.model.dir, false => Direction::Down  },
				Key::D => match view.model.dir == Direction::Left  { true => view.model.dir, false => Direction::Right },
				_ => view.model.dir
			}
		}
	}
}
