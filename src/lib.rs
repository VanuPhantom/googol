use std::path::PathBuf;

use gcp_auth::{AuthenticationManager, CustomServiceAccount, Token};
use tokio::sync::OnceCell;

/// The errors which are returned by *googol*'s functions
pub enum Error {
    /// Returned in case *googol* fails to load a service account from a file
    ServiceAccountError,
    /// Returned in case *googol* fails to acquire a token
    TokenError,
    /// Returned in case *googol* fails to load a service account from the environment
    EnvironmentError,
}

enum InitializationMethod<'a> {
    File(&'a str),
    Environment,
}

/// A wrapper around [`std::result::Result`] which is returned by *googol*'s functions
pub type Result<T> = std::result::Result<T, Error>;

/// The client through which *googol* interacts with [GCP](https://cloud.google.com)
pub struct Client<'a> {
    authentication_manager: OnceCell<AuthenticationManager>,
    scopes: &'a [&'a str],
    initialization_method: InitializationMethod<'a>,
}

impl<'a> Client<'a> {
    /// Initialize a client using a [service account key
    /// file](https://cloud.google.com/iam/docs/keys-create-delete)
    pub const fn from_file(path: &'a str, scopes: &'a [&'a str]) -> Client<'a> {
        Client {
            initialization_method: InitializationMethod::File(path),
            authentication_manager: OnceCell::const_new(),
            scopes,
        }
    }

    /// Initialize a client using [Application Default
    /// Credentials](https://cloud.google.com/docs/authentication/application-default-credentials)
    pub const fn from_environment(scopes: &'a [&'a str]) -> Client<'a> {
        Client {
            initialization_method: InitializationMethod::Environment,
            authentication_manager: OnceCell::const_new(),
            scopes,
        }
    }

    async fn get_authentication_manager(&'a self) -> Result<&'a AuthenticationManager> {
        Ok(match self.initialization_method {
            InitializationMethod::File(path) => {
                self.authentication_manager
                    .get_or_try_init(|| async {
                        let service_account = CustomServiceAccount::from_file(PathBuf::from(path))
                            .or(Err(Error::ServiceAccountError))?;

                        Ok(AuthenticationManager::from(service_account))
                    })
                    .await?
            }
            InitializationMethod::Environment => {
                self.authentication_manager
                    .get_or_try_init(|| async {
                        Ok(AuthenticationManager::new()
                            .await
                            .or(Err(Error::EnvironmentError))?)
                    })
                    .await?
            }
        })
    }

    /// Fetches an authentication token for the client's scopes
    pub async fn get_token(&self) -> Result<Token> {
        self.get_authentication_manager()
            .await?
            .get_token(self.scopes)
            .await
            .or(Err(Error::TokenError))
    }
}
