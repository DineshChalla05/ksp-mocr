use krpc_client::error::RpcError;
use krpc_client::services::space_center::{
    AutoPilot, Control, ReferenceFrame, SpaceCenter, Vessel,
};

pub struct VesselControl {
    control: Control,
    auto_pilot: AutoPilot,
    sas_before_engage: Option<bool>,
}

impl VesselControl {
    pub fn init_vessel_control(space_center: &SpaceCenter) -> Result<Self, RpcError> {
        let vessel = space_center.get_active_vessel()?;
        let control = vessel.get_control()?;
        let auto_pilot = vessel.get_auto_pilot()?;
        let orbit = vessel.get_orbit()?;
        let body = orbit.get_body()?;
        let surface_reference_frame: ReferenceFrame = body.get_reference_frame()?;

        auto_pilot.set_reference_frame(&surface_reference_frame)?;

        Ok(Self {
            control,
            auto_pilot,
            sas_before_engage: None,
        })
    }

    pub fn activate_next_stage(&self) -> Result<(), RpcError> {
        self.control.activate_next_stage()?;
        Ok(())
    }

    pub fn engage_autopilot(&mut self, pitch: f32, heading: f32) -> Result<(), RpcError> {
        // Only capture SAS state on the first engage since the last disengage
        if self.sas_before_engage.is_none() {
            self.sas_before_engage = Some(self.control.get_sas()?);
        }
        self.auto_pilot.target_pitch_and_heading(pitch, heading)?;
        self.auto_pilot.engage()?;
        Ok(())
    }

    pub fn disengage_autopilot(&mut self) -> Result<(), RpcError> {
        self.auto_pilot.disengage()?;
        if let Some(previous_sas) = self.sas_before_engage.take() {
            self.control.set_sas(previous_sas)?;
        }
        Ok(())
    }

    pub fn autopilot_error(&self) -> Result<f32, RpcError> {
        self.auto_pilot.get_error()
    }
}
