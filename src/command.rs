use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Control, SpaceCenter};

pub struct VesselControl {
    control: Control,
}

impl VesselControl {
    pub fn init_vessel_control(space_center: &SpaceCenter) -> Result<Self, RpcError> {
        let vessel = space_center.get_active_vessel()?;
        let control = vessel.get_control()?;
        Ok(Self { control })
    }

    pub fn activate_next_stage(&self) -> Result<(), RpcError> {
        self.control.activate_next_stage()?;
        // for now we are ignoring the VEc<Vessel>. But in the future, I will need it.
        Ok(())
    }
}
