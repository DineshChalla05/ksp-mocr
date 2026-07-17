use krpc_client::error::RpcError;
use krpc_client::services::space_center::{CelestialBody, Orbit, SpaceCenter};
use krpc_client::stream::Stream;
use std::fmt;
use std::fmt::Formatter;

pub enum BodyNames {
    Kerbol,
    Eve,
    Gilly,
    Dres,
    Mun,
    Kerbin,
    Minmus,
    Moho,
    Duna,
    Ike,
    Jool,
    Laythe,
    Bop,
    Pol,
    Vall,
    Tylo,
    Eeloo,
}

impl fmt::Display for BodyNames {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BodyNames::Kerbol => write!(f, "Kerbol"),
            BodyNames::Eve => write!(f, "Eve"),
            BodyNames::Gilly => write!(f, "Gilly"),
            BodyNames::Dres => write!(f, "Dres"),
            BodyNames::Mun => write!(f, "Mun"),
            BodyNames::Kerbin => write!(f, "Kerbin"),
            BodyNames::Minmus => write!(f, "Minmus"),
            BodyNames::Moho => write!(f, "Moho"),
            BodyNames::Duna => write!(f, "Duna"),
            BodyNames::Ike => write!(f, "Ike"),
            BodyNames::Jool => write!(f, "Jool"),
            BodyNames::Laythe => write!(f, "Laythe"),
            BodyNames::Bop => write!(f, "Bop"),
            BodyNames::Pol => write!(f, "Pol"),
            BodyNames::Vall => write!(f, "Vall"),
            BodyNames::Tylo => write!(f, "Tylo"),
            BodyNames::Eeloo => write!(f, "Eeloo"),
        }
    }
}

pub struct Telemetry {
    pub ut: Stream<f64>,
    pub apoapsis: Stream<f64>,
    pub periapsis: Stream<f64>,
    pub argument_of_periapsis: Stream<f64>,
    pub true_anomaly: Stream<f64>,
    pub body_radius: f64,
}

fn init_ut_stream(space_center: &SpaceCenter, rate: f32) -> Result<Stream<f64>, RpcError> {
    let ut_stream = space_center.get_ut_stream()?;
    ut_stream.set_rate(rate)?;
    Ok(ut_stream)
}

fn init_apoapsis_altitude_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let apoapsis_stream = orbit.get_apoapsis_altitude_stream()?;
    apoapsis_stream.set_rate(rate)?;
    Ok(apoapsis_stream)
}

fn init_periapsis_altitude_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let periapsis_stream = orbit.get_periapsis_altitude_stream()?;
    periapsis_stream.set_rate(rate)?;
    Ok(periapsis_stream)
}

fn init_argument_of_periapsis_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let arg_periapsis_stream = orbit.get_argument_of_periapsis_stream()?;
    arg_periapsis_stream.set_rate(rate)?;
    Ok(arg_periapsis_stream)
}

fn init_true_anomaly_stream(orbit: &Orbit, rate: f32) -> Result<Stream<f64>, RpcError> {
    let true_anomaly_stream = orbit.get_true_anomaly_stream()?;
    true_anomaly_stream.set_rate(rate)?;
    Ok(true_anomaly_stream)
}

fn get_body_radius(orbit: &Orbit) -> Result<f64, RpcError> {
    let body: CelestialBody = orbit.get_body()?;
    body.get_equatorial_radius()
}

pub fn get_telemetry(space_center: SpaceCenter) -> Result<Telemetry, RpcError> {
    let vessel = space_center.get_active_vessel()?;
    let orbit = vessel.get_orbit()?;
    Ok(Telemetry {
        ut: init_ut_stream(&space_center, 1f32)?,
        apoapsis: init_apoapsis_altitude_stream(&orbit, 1f32)?,
        periapsis: init_periapsis_altitude_stream(&orbit, 1f32)?,
        argument_of_periapsis: init_argument_of_periapsis_stream(&orbit, 1f32)?,
        true_anomaly: init_true_anomaly_stream(&orbit, 1f32)?,
        body_radius: get_body_radius(&orbit)?,
    })
}
