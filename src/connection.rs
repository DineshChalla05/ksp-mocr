use krpc_client::Client;
use krpc_client::error::RpcError;
use krpc_client::services::space_center::{Orbit, SpaceCenter};

const NAME: &str = "ksp-mocr";
const HOST: &str = "127.0.0.1";
const PORT1: u16 = 50000;
const PORT2: u16 = 50001;

pub struct ConnectionInfo {
    pub client: Client,
    pub space_center: SpaceCenter,
    pub orbit: Orbit,
}

pub fn init_connection() -> Result<SpaceCenter, RpcError> {
    let client = Client::new(NAME, HOST, PORT1, PORT2).unwrap();
    let space_center = SpaceCenter::new(client.clone());
    Ok(space_center)
}
