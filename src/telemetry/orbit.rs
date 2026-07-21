use krpc_client::error::RpcError;
use krpc_client::services::space_center::{CelestialBody, Orbit};
use krpc_client::stream::Stream;

pub fn init_apoapsis_altitude_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let apoapsis_stream = orbit.get_apoapsis_altitude_stream()?;
    apoapsis_stream.set_rate(rate)?;
    Ok(apoapsis_stream)
}

pub fn init_periapsis_altitude_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let periapsis_stream = orbit.get_periapsis_altitude_stream()?;
    periapsis_stream.set_rate(rate)?;
    Ok(periapsis_stream)
}

pub fn init_argument_of_periapsis_stream(
    orbit: &Orbit,
    rate: f32,
) -> Result<Stream<f64>, RpcError> {
    let arg_periapsis_stream = orbit.get_argument_of_periapsis_stream()?;
    arg_periapsis_stream.set_rate(rate)?;
    Ok(arg_periapsis_stream)
}

pub fn init_true_anomaly_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let true_anomaly_stream = orbit.get_true_anomaly_stream()?;
    true_anomaly_stream.set_rate(rate)?;
    Ok(true_anomaly_stream)
}

pub fn get_body_radius(orbit: &Orbit) -> Result<f64, RpcError> {
    let body: CelestialBody = orbit.get_body()?;
    body.get_equatorial_radius()
}

pub fn init_inclination_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let inclination_stream = orbit.get_inclination_stream()?;
    inclination_stream.set_rate(rate)?;
    Ok(inclination_stream)
}

pub fn init_period_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let period_stream = orbit.get_period_stream()?;
    period_stream.set_rate(rate)?;
    Ok(period_stream)
}

pub fn init_time_to_apoapsis_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let stream = orbit.get_time_to_apoapsis_stream()?;
    stream.set_rate(rate)?;
    Ok(stream)
}

pub fn init_time_to_periapsis_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let stream = orbit.get_time_to_periapsis_stream()?;
    stream.set_rate(rate)?;
    Ok(stream)
}