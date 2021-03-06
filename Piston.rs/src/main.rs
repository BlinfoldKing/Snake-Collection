extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::Rng;
use rand::thread_rng;

#[derive(Clone, PartialEq)]
enum Direction {
    UP, DOWN, LEFT, RIGHT
}

struct Game {
    pause: bool,
    gl: GlGraphics,
    snake: Snake,
    food: (i32, i32)
}

impl Game {

    fn reset (&mut self) {

        self.generate_food();
        self.snake = Snake {
            is_dead: false,
            body: LinkedList::from_iter((vec![(10, 10), (9, 10), (8, 10)]).into_iter()),
            dir: Direction::RIGHT
        }
    }

    fn render (&mut self, arg: &RenderArgs) {
        use graphics;

        let bg: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let foor_clr: [f32; 4] = [1.0, 0.0, 0.3, 1.0];
        let food = graphics::rectangle::square(
            (self.food.0 * 10) as f64,
            (self.food.1 * 10) as f64,
            10_f64
        );


        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(bg, gl)
        });

        self.gl.draw(arg.viewport(), |_c, gl| {
            let transform = _c.transform;
            graphics::rectangle(foor_clr, food, transform, gl);
        });


        self.snake.render(&mut self.gl, arg);
    }

    fn generate_food (&mut self) {
       let mut r = thread_rng();
       self.food.0 = r.gen_range(0, 50);
       self.food.1 = r.gen_range(0, 50);
    }

    fn update (&mut self) {

        if self.snake.is_dead {
            self.reset();
        }

        if !self.pause {
            self.snake.update();
        }

        if self.snake.eat(self.food) {
            self.generate_food();
            self.snake.grow();
        }
    }

    fn pressed (&mut self, button: &Button) {

        if button == &Button::Keyboard(Key::Space) {
            self.pause = !self.pause;
        }

        let last_dir = self.snake.dir.clone();

        self.snake.dir = match button {
            &Button::Keyboard(Key::Up)
                if last_dir != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down)
                if last_dir != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left)
                if last_dir != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right)
                if last_dir != Direction::LEFT => Direction::RIGHT,
            _ => last_dir
        }
    }

}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
    is_dead: bool
}

impl Snake {
    fn render (&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let color: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * 10) as f64,
                    (y * 10) as f64,
                    10_f64
                )
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square|
                    graphics::rectangle(color, square, transform, gl));
        })
    }

    fn update (&mut self) {

        if !self.is_dead {

            let mut new_head = (*self.body.front().expect("")).clone();

            if self.dir == Direction::RIGHT {
                new_head.0 += 1;
            }
            if self.dir == Direction::LEFT {
                new_head.0 -= 1;
            }
            if self.dir == Direction::UP {
                new_head.1 -= 1;
            }
            if self.dir == Direction::DOWN {
                new_head.1 += 1;
            }

            self.is_dead = new_head.0 < 0 || new_head.0 > 49 || new_head.1 < 0 || new_head.1 > 49;
            self.body.push_front(new_head);
            self.body.pop_back().unwrap();

            let mut iter = self.body.clone().into_iter();
            iter.next();
            loop {
                match iter.next() {
                    Some(x) => {
                        if x.0 == self.body.front().expect("").0 && x.1 == self.body.front().expect("").1 {
                            self.is_dead = true;
                        }
                    },
                    None => break,
                }
            }

            println!("{}", self.is_dead);



        }

    }

    fn eat (&mut self, food: (i32, i32)) -> bool{
        return (*self.body.front().expect("")).0 == food.0
        && (*self.body.front().expect("")).1 == food.1
    }

    fn grow (&mut self) {
        let tail = (*self.body.back().expect("")).clone();
        self.body.push_back((
            match self.dir {
                Direction::LEFT => tail.0 - 1,
                Direction::RIGHT => tail.0 + 1,
                _ => tail.0
            },
            match self.dir {
                Direction::UP => tail.1 - 1,
                Direction::DOWN => tail.1 + 1,
                _ => tail.1
            }
        ))
    }
}


fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Snake Game",
        [500, 500],
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        food: (30, 30),
        gl: GlGraphics::new(opengl),
        pause: false,
        snake: Snake {
            is_dead: false,
            body: LinkedList::from_iter((vec![(10, 10), (10, 11), (10, 12)]).into_iter()),
            dir: Direction::RIGHT
        }
    };

    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {
                game.pressed(&b.button);
            }
        }
    }

}
