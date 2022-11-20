use glam::DVec2;
use piston::UpdateArgs;

pub struct Blade {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub speed: f64,
    pub rotation: f64,
    pub direction: DVec2,
}

impl Blade {
    pub fn move_forward(&mut self, args: &UpdateArgs) {
        let pos = DVec2::new(self.x, self.y);
        let vec_speed = self.direction * self.speed * args.dt;
        let res_pos = pos + vec_speed;
        self.x = res_pos[0];
        self.y = res_pos[1];
        self.speed += 3.0 * self.speed * args.dt;
    }
}

impl Drop for Blade {
    fn drop(&mut self) {
        println!("Blade is dropped!");
    }
}
