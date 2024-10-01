use std::collections::BTreeMap;

use chrono::{Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::errors::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

impl JwtClaims {
    pub fn new(username: String, user_id: Uuid, role: String) -> Self {
        Self {
            sub: username,
            user_id,
            role,
            exp: (Utc::now() + Duration::days(1)).timestamp(),
            iat: Utc::now().timestamp(),
        }
    }

    pub fn to_tree_map(&self) -> BTreeMap<&'static str, String> {
        let mut claims = BTreeMap::new();
        claims.insert("sub", self.sub.clone());
        claims.insert("user_id", self.user_id.to_string());
        claims.insert("role", self.role.clone());
        claims.insert("exp", self.exp.to_string());
        claims.insert("iat", self.iat.to_string());
        claims
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now().timestamp();
        self.exp > now && self.iat <= now
    }
}

impl TryFrom<BTreeMap<String, String>> for JwtClaims {
    type Error = Error;

    fn try_from(claims: BTreeMap<String, String>) -> Result<Self> {
        let sub = claims
            .get("sub")
            .ok_or_else(|| Error::LoginFailed("Error in JWT formatting".to_string()))?;
        let user_id = claims
            .get("user_id")
            .ok_or_else(|| Error::LoginFailed("Error in JWT formatting".to_string()))?;
        let role = claims
            .get("role")
            .ok_or_else(|| Error::LoginFailed("Error in JWT formatting".to_string()))?;
        let exp = claims
            .get("exp")
            .ok_or_else(|| Error::LoginFailed("Error in JWT formatting".to_string()))?;
        let iat = claims
            .get("iat")
            .ok_or_else(|| Error::LoginFailed("Error in JWT formatting".to_string()))?;

        Ok(Self {
            sub: sub.clone(),
            user_id: Uuid::parse_str(user_id)
                .map_err(|_| Error::LoginFailed("Error in JWT formatting".to_string()))?,
            role: role.clone(),
            exp: exp
                .parse()
                .map_err(|_| Error::LoginFailed("Error in JWT formatting".to_string()))?,
            iat: iat
                .parse()
                .map_err(|_| Error::LoginFailed("Error in JWT formatting".to_string()))?,
        })
    }
}
