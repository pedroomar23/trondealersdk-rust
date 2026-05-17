pub mod register;

use serde_json::json;
// Register Request 
pub struct RegisterRq {
    name: String,
    webhook_url: String,
    webhook_secret: String, 
    payout_method: String,
    sweep_wallet_evm: String, 
    sweep_wallet_tron: String 
}

pub impl RegisterRq {
    fn fromJson(
        &self, 
        name: String, 
        webhook_url: String, 
        webhook_secret: String, 
        payout_method: String, 
        sweep_wallet_evm: String,
        sweep_wallet_tron: String
    ) -> RegisterRq {
        RegisterRq {
            name: self.name, 
            webhook_url: self.webhook_url,
            webhook_secret: self.webhook_secret,
            payout_method: self.payout_method,
            sweep_wallet_evm: self.sweep_wallet_evm,
            sweep_wallet_tron: self.sweep_wallet_tron
        }
     }

     fn toJson(
        name: String, 
        webhook_url: String, 
        webhook_secret: String, 
        payout_method: String, 
        sweep_wallet_evm: String,
        sweep_wallet_tron: String
    ) -> String {
        RegisterRq {
            name: "name",
            webhook_url: "webhook_url",
            webhook_secret: "webhook_secret",
            payout_method: "payout_method",
            sweep_wallet_evm: "sweep_wallet_evm",
            sweep_wallet_tron: "sweep_wallet_tron"
        }
     }
}

// Register Response 
pub struct RegisterResp {
    success: bool,
    client: Client
}

pub impl RegisterResp {
    fn fromJson(
        &self,
        success: bool,
        client: Client
    ) -> RegisterResp {
        RegisterResp {
            success: self.success,
            client: self.client
        }
    }

    fn toJson(
        success: bool,
        client: Client
    ) -> String {
        RegisterResp {
            success: "success",
            client: "client"
        }
    }
}

pub struct Client {
    id: String, 
    name: String,
    api_key: String, 
    webhook_url: String, 
    min_confirmations: nil,
    sweep_wallet_evm: String,
    sweep_wallet_tron: String, 
    payout_method: String,
    zelle_account: nil,
    qvapay_account: nil,
    is_active: bool,
    created_at: String 
}

pub impl Client {

}