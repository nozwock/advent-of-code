use anyhow::Result;

pub struct LanternFish {
    timer: u8,
}

impl LanternFish {
    pub fn new(timer: u8) -> Self {
        Self { timer }
    }
    pub fn pass_time_by_day(&mut self) -> Option<Self> {
        if self.timer == 0 {
            // reset timer
            self.timer = 6;
            return Some(LanternFish::new(8));
        }
        self.timer -= 1;
        None
    }
    /// Will create new LanternFish upon timer resets
    fn pass_time_by(days: usize) -> Option<Vec<Self>> {
        unimplemented!()
    }
}
