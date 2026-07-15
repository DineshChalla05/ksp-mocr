use krpc_client::{services::space_center::SpaceCenter, Client};
use krpc_client::error::RpcError;

fn init_connection() -> Result<SpaceCenter, RpcError>{
    let client = Client::new("ksp-mocr", "127.0.0.1", 50000, 50001).unwrap();

    let space_center = SpaceCenter::new(client.clone());

    let ship = space_center.get_active_vessel()?;

    match ship.get_crew()?.first() {
        Some(kerbal) => println!(
            "hello, {}. Welcome to {}",
            kerbal.get_name()?,
            ship.get_name()?
        ),
        None => println!("{} is unkerbaled!", ship.get_name()?),
    };
    Ok(space_center)
}
fn main() {
    let result = init_connection();
    match result {
        Ok(_) => println!("Okay!"),
        Err(a) => println!("{}", a.to_string())
    }
}