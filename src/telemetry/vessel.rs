use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Flight, ReferenceFrame, Vessel, VesselSituation};
use krpc_client::stream::Stream;

pub fn init_flight(vessel: &Vessel, reference_frame: &ReferenceFrame) -> Result<Flight, RpcError> {
    vessel.flight(Some(reference_frame))
}

pub fn init_speed_stream(flight: &Flight, rate: f32) -> Result<Stream<f64>, RpcError> {
    let speed_stream = flight.get_speed_stream()?;
    speed_stream.set_rate(rate)?;
    Ok(speed_stream)
}

pub fn init_vertical_speed_stream(flight: &Flight, rate: f32) -> Result<Stream<f64>, RpcError> {
    let vertical_speed_stream = flight.get_vertical_speed_stream()?;
    vertical_speed_stream.set_rate(rate)?;
    Ok(vertical_speed_stream)
}

pub fn init_horizontal_speed_stream(flight: &Flight, rate: f32) -> Result<Stream<f64>, RpcError> {
    let horizontal_speed_stream = flight.get_horizontal_speed_stream()?;
    horizontal_speed_stream.set_rate(rate)?;
    Ok(horizontal_speed_stream)
}

pub fn init_mass_stream(vessel: &Vessel, rate: f32) -> Result<Stream<f32>, RpcError> {
    let mass_stream = vessel.get_mass_stream()?;
    mass_stream.set_rate(rate)?;
    Ok(mass_stream)
}

pub fn init_liquid_fuel_stream(vessel: &Vessel, rate: f32) -> Result<Stream<f32>, RpcError> {
    let resources = vessel.get_resources()?;
    let fuel_stream = resources.amount_stream("LiquidFuel".to_string())?;
    fuel_stream.set_rate(rate)?;
    Ok(fuel_stream)
}


pub fn init_pitch_stream(flight: &Flight, rate: f32) -> Result<Stream<f32>, RpcError> {
    let pitch_stream = flight.get_pitch_stream()?;
    pitch_stream.set_rate(rate)?;
    Ok(pitch_stream)
}

pub fn init_heading_stream(flight: &Flight, rate: f32) -> Result<Stream<f32>, RpcError> {
    let heading_stream = flight.get_heading_stream()?;
    heading_stream.set_rate(rate)?;
    Ok(heading_stream)
}

pub fn init_roll_stream(flight: &Flight, rate: f32) -> Result<Stream<f32>, RpcError> {
    let roll_stream = flight.get_roll_stream()?;
    roll_stream.set_rate(rate)?;
    Ok(roll_stream)
}

pub fn init_dynamic_pressure_stream(flight: &Flight, rate: f32) -> Result<Stream<f32>, RpcError> {
    let q_stream = flight.get_dynamic_pressure_stream()?;
    q_stream.set_rate(rate)?;
    Ok(q_stream)
}

pub fn init_g_force_stream(flight: &Flight, rate: f32) -> Result<Stream<f32>, RpcError> {
    let g_stream = flight.get_g_force_stream()?;
    g_stream.set_rate(rate)?;
    Ok(g_stream)
}
// mission elapsed time cause I keep forgetting.
pub fn init_met_stream(vessel: &Vessel, rate: f32) -> Result<Stream<f64>, RpcError> {
    let met_stream = vessel.get_met_stream()?;
    met_stream.set_rate(rate)?;
    Ok(met_stream)
}

pub fn init_situation_stream(vessel: &Vessel, rate: f32) -> Result<Stream<VesselSituation>, RpcError> {
    let situation_stream = vessel.get_situation_stream()?;
    situation_stream.set_rate(rate)?;
    Ok(situation_stream)
}

pub fn init_thrust_stream(vessel: &Vessel, rate: f32) -> Result<Stream<f32>, RpcError> {
    let thrust_stream = vessel.get_thrust_stream()?;
    thrust_stream.set_rate(rate)?;
    Ok(thrust_stream)
}