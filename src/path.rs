use opengl_graphics::{
    GlGraphics, 
    OpenGL
};
use graphics::{
    Context,
    Line,
    Ellipse,
};

pub struct Path {
    pub splines: Vec<Spline>,
    pub color: [f32; 4],
    pub radius: f64,
}

impl Path {
    pub fn new(color: [f32; 4], radius: f64) -> Self {
        Path {
            splines: Vec::new(),
            color: color,
            radius: radius,
        }
    }

    pub fn from_splines(splines: Vec<Spline>, color: [f32; 4], radius: f64) -> Self {
        Path {
            splines: splines,
            color: color,
            radius: radius,
        }
    }

    pub fn add_spline(&mut self, spline: Spline) {
        self.splines.push(spline);
    }
}

#[derive(Debug)]
pub struct Spline {
    pub control_points: Vec<[f64; 2]>,
    points: Vec<[f64; 2]>,
    pub resolution: usize,
}

impl Spline {
    pub fn new(resolution: usize, control_points: Vec<[f64; 2]>) -> Self {
        let mut spline = Spline {
            control_points: control_points,
            points: Vec::new(),
            resolution: resolution,
        };
        spline.compute_path();
        spline
    }

    pub fn add_control_point(&mut self, point: [f64; 2]) {
        self.control_points.push(point);
        self.compute_path();
    }

    pub fn compute_path(&mut self) {
        self.points.clear();
        
        if self.control_points.len() < 2 {
            self.points = self.control_points.clone();
            return;
        }
       
        if self.control_points.len() == 2 {
            self.points.push(self.control_points[0]);
            self.points.push(self.control_points[1]);
            return;
        }

        if self.control_points.len() == 3 {
            self.points = self._compute_path(self.control_points[0], self.control_points[1], self.control_points[2]);
            return;
        }

        let p1 = self._compute_path(self.control_points[0], self.control_points[1], self.control_points[2]);
        let p2 = self._compute_path(self.control_points[1], self.control_points[2], self.control_points[3]);

        self.points = self._compute_path_over(p1, p2);
    }

    fn _compute_path_over(&mut self, pr0: Vec<[f64; 2]>, pr1: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
        let mut points: Vec<[f64; 2]> = Vec::new();
        let mut t = 0.0;
        let mut i = 0;
        let step = 1.0 / self.resolution as f64;
        while t <= 1.0 {
            points.push(self.lerp(t, pr0[i], pr1[i]));
            t += step;
            i += 1;
        }
        points
    }

    fn _compute_path(&mut self, p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> Vec<[f64; 2]> {
        let mut t = 0.0;
        let mut m0: [f64; 2];
        let mut m1: [f64; 2];
        let mut q0: [f64; 2];
        let step = 1.0 / self.resolution as f64;
        let mut points: Vec<[f64; 2]> = Vec::new();
        while t <= 1.0 {
            m0 = self.lerp(t, p0, p1);
            m1 = self.lerp(t, p1, p2);
            q0 = self.lerp(t, m0, m1);
            points.push(q0);
            t += step;
        }
        points.push(p2);
        points
    }

    fn lerp_points(&mut self, p0: [f64; 2], p1: [f64; 2]) -> Vec<[f64; 2]> {
        let mut t = 0.0;
        let mut points: Vec<[f64; 2]> = Vec::new();
        let step = 1.0 / self.resolution as f64;
        while t <= 1.0 {
            points.push(self.lerp(t, p0, p1));
            t += step;
        }
        points
    }

    fn lerp(&mut self, t: f64, p0: [f64; 2], p1: [f64; 2]) -> [f64; 2] {
        let x = p0[0] + (p1[0] - p0[0]) * t;
        let y = p0[1] + (p1[1] - p0[1]) * t;
        [x, y]
    }
}

pub trait DrawSpline {
    fn draw_spline(&mut self, spline: &Spline, color: [f32; 4], radius: f64, c: &Context);
}

impl DrawSpline for GlGraphics {
    fn draw_spline(&mut self, spline: &Spline, color: [f32; 4], radius: f64, c: &Context) {
        if spline.control_points.len() < 2 {
            return;
        }

        let mut lines: Vec<[f64; 4]> = Vec::new();
        let mut last_point: [f64; 2] = spline.control_points[0];

        for (i, point) in spline.points[1..].iter().enumerate() {
            let line = [last_point[0], last_point[1], point[0], point[1]];
            lines.push(line);
            last_point = *point;
        }

        for line in lines.iter() {
            Line::new(color, radius).draw(*line, &c.draw_state, c.transform, self);
        }

        for point in spline.control_points.iter() {
            let rect = [point[0] - radius * 2.0, point[1] - radius * 2.0, radius * 4.0, radius * 4.0];
            Ellipse::new([1.0, 0.0, 1.0, 1.0]).draw(rect, &c.draw_state, c.transform, self);
        }
    }
}
