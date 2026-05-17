pub mod networks; 

// Networks 
pub struct NetworksResp {
    success: bool, 
    networks: [Networks]
}

pub struct Networks {
    key: String, 
    family: String, 
    label: String, 
    network_param: String, 
    native_token: String, 
    default_min_confirmations: int,
    assets: [Assets]
}

pub struct Assets {
    symbol: String, 
    decimals: int, 
    contract: String 
}