extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

#[derive(Clone, PartialEq)]
enum Direction {
    UP, DOWN, LEFT, RIGHT
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render (&mut self, arg: &RenderArgs) {
        use graphics;

        let Bg: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(Bg, gl)
        });

        self.snake.render(&mut self.gl, arg);
    }
    
    fn update (&mut self) {
        self.snake.update();        
    }

     fn pressed (&mut self, button: &Button) {

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
    posX: i32,
    posY: i32,
    dir: Direction
}

impl Snake {
    fn render (&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
        let square = graphics::rectangle::square(
            (self.posX * 10) as f64, 
            (self.posY * 10) as f64,
            10_f64
            );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(COLOR, square, transform, gl);
        })
    }

    fn ChangeDir (&mut self, dir: Direction) {

    }

    fn update (&mut self) {
        if (self.dir == Direction::RIGHT) {
            self.posX += 1;
        }
        if (self.dir == Direction::LEFT) {
            self.posX -= 1;
        }
        if (self.dir == Direction::UP) {
            self.posY -= 1;
        }
        if (self.dir == Direction::DOWN) {
            self.posY += 1;
        }
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
        gl: GlGraphics::new(opengl),
        snake: Snake {
            posX: 12,
            posY: 10,
            dir: Direction::RIGHT        
        }
    };

    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {
                game.pressed(&b.button);
            }
        }
    }
}
