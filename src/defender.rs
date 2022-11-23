use std::f32::consts::PI;

use glam::DVec2;
use piston::UpdateArgs;

use crate::blade::Blade;

#[derive(Debug)]
pub struct Defender {
    pub position: DVec2,
    pub size: f64,
    pub rotation: f64,
    pub speed: f64,
    pub vec_speed: DVec2,
}

impl Defender {
    pub fn collide(&self, _colliding_blade: &Blade) {}

    pub fn evolve() {}

    pub fn move_to_player(&mut self, player_x: f64, player_y: f64, args: &UpdateArgs) {
        let p_pos = DVec2::new(player_x, player_y);
        let pos = DVec2::new(self.position.x, self.position.y);

        let direction = p_pos - pos;
        let angle = f64::atan2(direction.y, direction.x) * 180.0 / f64::from(PI);
        self.rotation = angle;
        self.vec_speed = direction.normalize() * self.speed * args.dt;
        let res_pos = pos + self.vec_speed;
        self.position = res_pos;
    }
}

impl Drop for Defender {
    fn drop(&mut self) {
        println!("Defender is dropped!");
    }
}
