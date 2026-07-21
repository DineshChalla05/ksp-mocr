pub struct SessionGuard {
    last_met: Option<f64>,
}

impl SessionGuard {
    pub fn init_session_guard() -> Self {
        Self { last_met: None }
    }

    pub fn tick(&mut self, met: f64) -> bool {
        let discontinuity = matches!(self.last_met, Some(last) if met < last);
        self.last_met = Some(met);
        discontinuity
    }
}
