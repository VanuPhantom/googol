use std::path::Path;

use gcp_auth::{AuthenticationManager, CustomServiceAccount, Token};

pub enum Error {
    ServiceAccountError,
    TokenError,
    EnvironmentError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client<'a> {
    authentication_manager: AuthenticationManager,
    scopes: &'a [&'a str],
}

impl<'a> Client<'a> {
    pub async fn from_file<T>(path: T, scopes: &'a [&'a str]) -> Result<Client<'a>>
    where
        T: AsRef<Path>,
    {
        let service_account =
            CustomServiceAccount::from_file(path).or(Err(Error::ServiceAccountError))?;

        Ok(Client {
            authentication_manager: AuthenticationManager::from(service_account),
            scopes,
        })
    }

    pub async fn from_environment(scopes: &'a [&'a str]) -> Result<Client<'a>> {
        Ok(Client {
            authentication_manager: AuthenticationManager::new()
                .await
                .or(Err(Error::EnvironmentError))?,
            scopes,
        })
    }

    pub async fn get_token(self) -> Result<Token> {
        self.authentication_manager
            .get_token(self.scopes)
            .await
            .or(Err(Error::TokenError))
    }
}
