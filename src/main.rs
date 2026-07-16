use krpc_client::{services::space_center::SpaceCenter, Client};
use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Orbit, Vessel};
use krpc_client::stream::Stream;
use std::fmt;
use std::fmt::Formatter;

enum BodyNames {
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

struct ConnectionInfo {
    client: Client,
    space_center: SpaceCenter,
    orbit: Orbit,
}

struct Telemetry {
    ut: Stream<f64>,
    apoapsis: Stream<f64>,
}

fn init_connection() -> Result<SpaceCenter, RpcError> {
    let client = Client::new("ksp-mocr", "127.0.0.1", 50000, 50001).unwrap();
    let space_center = SpaceCenter::new(client.clone());
    Ok(space_center)
}

fn init_ut_stream(space_center: &SpaceCenter, rate: f32) -> Result<Stream<f64>, RpcError> {
    let ut_stream = space_center.get_ut_stream()?;
    ut_stream.set_rate(rate)?;
    Ok(ut_stream)
}

fn init_apoapsis_altitude_stream(space_center: &SpaceCenter, vessel: Vessel, rate: f32) -> Result<Stream<f64>, RpcError> {
    let orbit = vessel.get_orbit()?;
    let apoapsis_stream = orbit.get_apoapsis_altitude_stream()?;
    apoapsis_stream.set_rate(rate)?;
    Ok(apoapsis_stream)
}

fn get_telemetry(space_center: SpaceCenter) -> Result<Telemetry, RpcError> {
    let vessel = space_center.get_active_vessel()?;
    Ok(
        Telemetry {
            ut: init_ut_stream(&space_center, 1f32)?,
            apoapsis: init_apoapsis_altitude_stream(&space_center, vessel, 1f32)?,
        }
    )
}

fn info_loop() -> Result<(), RpcError> {
    let result = init_connection()?;
    let telemetry = get_telemetry(result)?;
    loop {
        println!("{}", telemetry.ut.get()?);
        println!("{}", telemetry.apoapsis.get()?);
        println!("------------------------")
    }
    Ok(())
}
fn main() {
    match info_loop() {
        Ok(_) => println!("Okay!"),
        Err(a) => println!("{}", a.to_string())
    }
}