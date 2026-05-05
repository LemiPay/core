use uuid::Uuid;
use crate::application::auth::traits::web3_auth::Web3AuthTrait;

pub struct Web3Auth {
    
}

impl Web3Auth {
    pub fn new() -> Self {Self{}}
}

impl Web3AuthTrait for Web3Auth {
    fn generate_nonce(&self) -> String {
        Uuid::new_v4().to_string()
    }
}