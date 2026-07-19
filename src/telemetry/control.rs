use eframe::wgpu::wgc::binding_model::BindingZone::Stage;
use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Control, Vessel};
use krpc_client::stream::Stream;

pub fn init_control(vessel: &Vessel) -> Result<Control, RpcError> {
    vessel.get_control()
}

pub fn init_current_stage_stream(control: &Control, rate: f32) -> Result<Stream<i32>, RpcError> {
    let stage_stream = control.get_current_stage_stream()?;
    stage_stream.set_rate(rate)?;
    Ok(stage_stream)
}

pub fn init_throttle_stream(control: &Control, rate: f32) -> Result<Stream<f32>, RpcError> {
    let throttle_stream = control.get_throttle_stream()?;
    throttle_stream.set_rate(rate)?;
    Ok(throttle_stream)
}