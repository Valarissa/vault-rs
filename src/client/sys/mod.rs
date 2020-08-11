use serde::de::DeserializeOwned;
use super::VaultClient;

/// For mounting new endpoints to vault
pub mod mount;

impl<T> VaultClient<T>
where
    T: DeserializeOwned,
{
    /// Create new sys object
    pub fn sys(&self) -> Sys<T> {
        Sys::<T>{
            client: self,
        }
    }
}

/// Sys mount object
#[derive(Debug)]
pub struct Sys<'a, T> {
    client: &'a VaultClient<T>,
}

impl<T> Sys<'_, T>
where T: DeserializeOwned {
    /// Does things
    pub fn foo(&self){
        println!("Token: {}", self.client.token);
    }
}