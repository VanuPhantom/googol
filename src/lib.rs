use std::path::{PathBuf};

use gcp_auth::{AuthenticationManager, CustomServiceAccount, Token};
use tokio::sync::OnceCell;

pub enum Error {
    ServiceAccountError,
    TokenError,
    EnvironmentError,
}

enum InitializationMethod<'a>
{
    File(&'a str),
    Environment,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client<'a>
{
    authentication_manager: OnceCell<AuthenticationManager>,
    scopes: &'a [&'a str],
    initialization_method: InitializationMethod<'a>,
}

impl<'a> Client<'a>
{
    pub const fn from_file(path: &'a str, scopes: &'a [&'a str]) -> Client<'a>
    {
        Client {
            initialization_method: InitializationMethod::File(path),
            authentication_manager: OnceCell::const_new(),
            scopes,
        }
    }

    pub const fn from_environment(scopes: &'a [&'a str]) -> Client<'a> {
        Client {
            initialization_method: InitializationMethod::Environment,
            authentication_manager: OnceCell::const_new(),
            scopes,
        }
    }

    async fn get_authentication_manager(&'a self) -> Result<&'a AuthenticationManager> {
        Ok(match &self.initialization_method {
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

    pub async fn get_token(&self) -> Result<Token> {
        self.get_authentication_manager()
            .await?
            .get_token(self.scopes)
            .await
            .or(Err(Error::TokenError))
    }
}
