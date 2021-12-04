#[derive(Debug, Clone, Copy)]
pub struct Account {
    pub id: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn add_available_by(&mut self, delta: f32) {
        self.available += delta;
    }

    pub fn add_held_by(&mut self, delta: f32) {
        self.held += delta;
    }

    pub fn add_total_by(&mut self, delta: f32) {
        self.total += delta;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{:.4},{}",
            self.id, self.available, self.held, self.total, self.locked
        )
    }
}
