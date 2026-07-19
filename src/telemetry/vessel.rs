use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Flight, ReferenceFrame, Vessel};
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

