use crate::html::HtmlDom;
use js_sys::Math::{cos, sin, tan};
use std::f64::consts::FRAC_PI_4;
use web_commons::log;

pub struct AppState {
    pub html: HtmlDom,

    pub theta: f64,
    pub last_x_pos: f64,
    point_b: [f64; 2],
    point_c: [f64; 2],
}

impl AppState {
    pub fn create(html: HtmlDom) -> Self {
        let theta = FRAC_PI_4;
        let point_b = Self::calc_point_b(theta);
        let point_c = Self::calc_point_c(point_b[0]);
        log(std::format!("point_b: {:?}", point_b).as_str());
        log(std::format!("point_c: {:?}", point_c).as_str());
        Self {
            html,
            theta,
            last_x_pos: 0.0,
            point_b,
            point_c,
        }
    }

    pub fn get_point_a(&self) -> [f64; 2] {
        [0.0, 0.0]
    }

    fn calc_point_b(theta: f64) -> [f64; 2] {
        [
            cos(theta) * Self::get_dist_c(),
            sin(theta) * Self::get_dist_c(),
        ]
    }

    fn calc_point_c(x: f64) -> [f64; 2] {
        [x, 0.0]
    }

    pub fn get_point_b(&self) -> [f64; 2] {
        self.point_b
    }

    pub fn get_point_c(&self) -> [f64; 2] {
        [self.get_point_b()[0], 0.0]
    }

    pub fn update_points(&mut self) {
        self.point_b = Self::calc_point_b(self.theta);
        self.point_c = Self::calc_point_c(self.point_b[0]);
    }

    pub fn get_dist_c() -> f64 {
        100.0
    }

    pub fn get_sin(&self) -> f64 {
        sin(self.theta)
    }

    pub fn get_cos(&self) -> f64 {
        cos(self.theta)
    }

    pub fn get_tan(&self) -> f64 {
        tan(self.theta)
    }

    pub fn get_theta(&self) -> f64 {
        self.theta
    }
}
