use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Authentication {
    pub enabled: bool,
    pub root_token: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Configuration {
    pub authentication: Authentication,
}

impl Configuration {
    pub fn new_with_enabled(root_token: String, secret_key: String) -> Self {
        Configuration {
            authentication: Authentication {
                enabled: true,
                root_token,
                secret_key,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}
