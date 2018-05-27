extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


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
}


fn main() {
    println!("Hello World!");

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

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r)
        }
    }
}
