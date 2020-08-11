use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde_json::{
    map::Map,
    Value,
};
use super::Sys;
use crate::client;
use crate::client::error::{
    Error,
    Result,
};
use crate::client::VaultResponse;

impl<T> Sys<'_, T>
where T: DeserializeOwned {
    /// Does things
    pub fn list_mounts(&self) -> Result<HashMap<String, Map<String, Value>>> {
        let client = self.client;
        let res = client.get::<_, String>("/v1/sys/mounts", None)?;
        let decoded: VaultResponse<HashMap<String, Map<String, Value>>> = client::parse_vault_response(res)?;
        match decoded.data {
            Some(data) => Ok(data),
            _ => Err(Error::Vault(format!(
                "No mounts found in response: `{:#?}`",
                decoded
            ))),
        }
    }
}