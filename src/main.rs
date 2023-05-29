use opengl_graphics::{
    GlGraphics, 
    OpenGL,
    GlyphCache,
};
use piston_window::{
    PistonWindow, 
    WindowSettings, 
    RenderArgs, 
    UpdateArgs, 
    UpdateEvent, 
    RenderEvent,
    Graphics,
    Transformed,
    TextureSettings,
};
use graphics::{
    Context, 
    Rectangle,
    Text
};
use graphics::types::FontSize;
use graphics::rectangle::rectangle_by_corners;

mod path;
mod traveller;
use crate::path::*;
use crate::traveller::*;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


pub trait DrawRectangle {
    fn draw_rect(&mut self, rect: [f64; 4], color: [f32; 4], c: &Context);
}


impl DrawRectangle for GlGraphics {
    fn draw_rect(&mut self, rect: [f64; 4], color: [f32; 4], c: &Context) {
        Rectangle::new(color).draw(rect, &c.draw_state, c.transform, self);
    }
}

struct AppState {
    paths: Vec<Path>,
    travellers: Vec<Traveller>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            paths: Vec::new(),
            travellers: Vec::new(),
        }
    }
}

struct App {
    gl: GlGraphics,
    state: AppState
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        //let r = rectangle_by_corners(15.0, 15.0, 75.0, 100.0);
        let paths = &self.state.paths;

        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);

            for path in paths.iter() {
                for spline in path.splines.iter() {
                    g.draw_spline(spline, path.color, path.radius, &c);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) { }
}

fn init(state: &mut AppState) {
    let lines = vec![
        vec![[100.0, 100.0], [350.0, 100.0], [500.0, 200.0]],
        vec![[500.0, 200.0], [650.0, 300.0], [600.0, 400.0]],
        vec![[600.0, 400.0], [550.0, 500.0], [400.0, 500.0], [300.0, 400.0]],
    ];

    let mut splines: Vec<path::Spline> = Vec::new();

    for line in lines.iter() {
        splines.push(
            path::Spline::new(10, line.to_vec())
        );
    }

    state.paths.push(path::Path::from_splines(splines, WHITE, 1.0));
}

fn run_loop(app: &mut App, w: &mut PistonWindow) {
    while let Some(e) = w.next() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: PistonWindow = WindowSettings::new("Traffic", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gl = GlGraphics::new(opengl);

    let mut app = App { gl, state: AppState::new() };
    init(&mut app.state);
    run_loop(&mut app, &mut window);
}
