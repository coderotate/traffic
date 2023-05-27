use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{
    PistonWindow, 
    Window, 
    WindowSettings, 
    RenderArgs, 
    UpdateArgs, 
    UpdateEvent, 
    RenderEvent,
    Graphics
};
use graphics::{Context, Rectangle};
use graphics::rectangle::rectangle_by_corners;

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

struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let r = rectangle_by_corners(15.0, 15.0, 75.0, 100.0);

        self.gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
            g.draw_rect(r, WHITE, &c);
        });
    }

    fn update(&mut self, args: &UpdateArgs) { }
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

    let mut app = App { gl };
    run_loop(&mut app, &mut window);
}
