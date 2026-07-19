mod orbit;
mod vessel;

use krpc_client::error::RpcError;
use krpc_client::services::space_center::{CelestialBody, SpaceCenter};
use krpc_client::stream::Stream;
use std::fmt;
use std::fmt::Formatter;

pub enum BodyNames {
    Kerbol, Eve, Gilly, Dres, Mun, Kerbin, Minmus, Moho, Duna, Ike,
    Jool, Laythe, Bop, Pol, Vall, Tylo, Eeloo,
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
    pub inclination: Stream<f64>,
    pub surface_speed: Stream<f64>,
    pub vertical_speed: Stream<f64>,
    pub horizontal_speed: Stream<f64>,
    pub mass: Stream<f32>,
    pub liquid_fuel: Stream<f32>,
}

fn init_ut_stream(space_center: &SpaceCenter, rate: f32) -> Result<Stream<f64>, RpcError> {
    let ut_stream = space_center.get_ut_stream()?;
    ut_stream.set_rate(rate)?;
    Ok(ut_stream)
}

pub fn get_telemetry(space_center: SpaceCenter) -> Result<Telemetry, RpcError> {
    let vessel = space_center.get_active_vessel()?;
    let orbit = vessel.get_orbit()?;
    let body: CelestialBody = orbit.get_body()?;
    let surface_reference_frame = body.get_reference_frame()?;
    let flight = vessel::init_flight(&vessel, &surface_reference_frame)?;

    Ok(Telemetry {
        ut: init_ut_stream(&space_center, 1f32)?,
        apoapsis: orbit::init_apoapsis_altitude_stream(&orbit, 1f32)?,
        periapsis: orbit::init_periapsis_altitude_stream(&orbit, 1f32)?,
        argument_of_periapsis: orbit::init_argument_of_periapsis_stream(&orbit, 1f32)?,
        true_anomaly: orbit::init_true_anomaly_stream(&orbit, 1f32)?,
        body_radius: orbit::get_body_radius(&orbit)?,
        inclination: orbit::init_inclination_stream(&orbit, 1f32)?,
        surface_speed: vessel::init_speed_stream(&flight, 1f32)?,
        vertical_speed: vessel::init_vertical_speed_stream(&flight, 1f32)?,
        horizontal_speed: vessel::init_horizontal_speed_stream(&flight, 1f32)?,
        mass: vessel::init_mass_stream(&vessel, 1f32)?,
        liquid_fuel: vessel::init_liquid_fuel_stream(&vessel, 1f32)?,
    })
}