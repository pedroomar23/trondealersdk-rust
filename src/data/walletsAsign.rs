pub mod walletsAsign;

// Wallet Asign Request
pub struct WalletAsignRq {
    label: String 
}

// Wallet Asign Response 
pub struct WalletAsignResp {
    success: bool 
}

pub struct Wallet {
    id: String, 
    address: String,
    label: String, 
    status: String, 
    single_use: String,
    created_at: String 
}