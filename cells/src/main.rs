extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod ca;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use ca::*;


pub struct App {
    gl: GlGraphics,
    aut: ElementaryCellularAutomaton
}

impl App {
    fn render(&mut self, args: &RenderArgs, history: &Vec<Vec<bool>>) {
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 5.0);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for y in 0..history.len() {
                for x in 0..history[y].len() {
                    let transform = c.transform
                        .trans((x * 5) as f64, (y * 5) as f64);
                    if history[y][x] {
                        rectangle(BLACK, square, transform, gl);
                    }
                }
            }
        });
    }
    fn update(&mut self, args: &UpdateArgs, history: &mut Vec<Vec<bool>>) {
        if history.len() > 160 {
            return;
        }
        let last_state = if history.len() > 0 {
            match history.last() {
                Some(ref state) => state.to_vec(),
                None => vec![],
            }
        } else {
            vec![]
        };
        history.push(self.aut.successor(&last_state))
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Rule 30",
        [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        aut: ElementaryCellularAutomaton {
            rule: 120,
            size: 160,
            wrap: true,
            default: false,
        }
    };
    let mut history = vec![vec![
        false, false, false, true, true, false, false, false, true,
        false, false, false, true, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true,
        false, false, false, false, true, true, false, false, true,
        false, false, false, true, true, false, false, false, true,
        false, false, false, true, true, false, false, false, true,
        false, false, false, true, true, false, false, false, true,
        false, false, false, true, true, false, false, false,
        false, false, false, true, true, false, false, true,
        false, false, false, true, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true,
        false, false, false, false, true, false, false, true, true,
        false, false, false, true, false, false, false, true, false,
        false, false, true, true, false, false, false, true, true,
        false, false, true, true, false, false, false, true, false,
        false, false, true, true, false, false, false, true, true,
    ]];
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &history);
        }
        if let Some(u) = e.update_args() {
            app.update(&u, &mut history);
        }
    }
}
