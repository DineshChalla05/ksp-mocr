use krpc_client::services::space_center::VesselSituation;

pub struct FlightEvent {
    pub mission_elapsed_time: f64,
    pub message: String,
}

pub struct FlightState {
    last_stage: Option<i32>,
    last_situation: Option<VesselSituation>,
    peak_dynamic_pressure: f32,
    declining_ticks: u32,
    max_q_event_logged: bool,
    thrust_active: Option<bool>,
    peak_g_force: f32,
    events: Vec<FlightEvent>,
}

impl FlightState {
    pub fn init_flight_state() -> Self {
        Self {
            last_stage: None,
            last_situation: None,
            peak_dynamic_pressure: 0.0,
            declining_ticks: 0,
            max_q_event_logged: false,
            thrust_active: None,
            peak_g_force: 0.0,
            events: Vec::new(),
        }
    }

    fn check_stage(&mut self, mission_elapsed_time: f64, current_stage: i32) {
        match self.last_stage {
            None => self.last_stage = Some(current_stage),
            Some(prev) if (prev != current_stage) => {
                self.events.push(FlightEvent {
                    mission_elapsed_time,
                    message: format!("Stage separation: {} -> {}", prev, current_stage),
                });
                self.last_stage = Some(current_stage);
            }
            _ => {}
        }
    }

    fn check_situation(&mut self, mission_elapsed_time: f64, situation: VesselSituation) {
        match self.last_situation {
            None => self.last_situation = Some(situation),
            Some(prev) if prev != situation => {
                self.events.push(FlightEvent {
                    mission_elapsed_time,
                    message: format!("Situation changed: {:?} -> {:?}", prev, situation),
                });
                self.last_situation = Some(situation);
            }
            _ => {}
        }
    }

    fn track_max_q(&mut self, mission_elapsed_time: f64, dynamic_pressure: f32) {
        if self.max_q_event_logged {
            return;
        }

        if dynamic_pressure > self.peak_dynamic_pressure {
            self.peak_dynamic_pressure = dynamic_pressure;
            self.declining_ticks = 0;
        } else if self.peak_dynamic_pressure > 0.0 {
            self.declining_ticks += 1;
            if self.declining_ticks >= 3 {
                self.events.push(FlightEvent {
                    mission_elapsed_time,
                    message: format!("Max Q reached: {:.0} Pa", self.peak_dynamic_pressure),
                });
                self.max_q_event_logged = true;
            }
        }
    }

    fn check_engine_cutoff(&mut self, mission_elapsed_time: f64, thrust: f32) {
        let active = thrust > 1.0;
        match self.thrust_active {
            None => self.thrust_active = Some(active),
            Some(true) if !active => {
                self.events.push(FlightEvent {
                    mission_elapsed_time,
                    message: "Engine cutoff, no thrust".to_string(),
                });
                self.thrust_active = Some(false);
            }
            Some(false) if active => {
                self.events.push(FlightEvent {
                    mission_elapsed_time,
                    message: "Engine ignition, thrust activated".to_string(),
                });
                self.thrust_active = Some(true);
            }
            _ => {}
        }
    }

    fn track_peak_g_force(&mut self, g_force: f32) {
        if g_force > self.peak_g_force {
            self.peak_g_force = g_force;
        }
    }

    pub fn peak_g_force(&self) -> f32 {
        self.peak_g_force
    }

    pub fn events(&self) -> &[FlightEvent] {
        &self.events
    }

    pub fn tick(
        &mut self,
        met: f64,
        current_stage: i32,
        situation: VesselSituation,
        dynamic_pressure: f32,
        thrust: f32,
        g_force: f32,
    ) {
        self.check_stage(met, current_stage);
        self.check_situation(met, situation);
        self.track_max_q(met, dynamic_pressure);
        self.check_engine_cutoff(met, thrust);
        self.track_peak_g_force(g_force);
    }
}
