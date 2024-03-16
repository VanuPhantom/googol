# googol

## TLDR
A library to reduce the boilerplate needed to call Google APIs.

## Example usage
```rust
const SCOPES: &'static [&'static str] = &["https://www.googleapis.com/auth/firebase.messaging"];

#[cfg(debug_assertions)]
static GCP_AUTHENTICATION_CLIENT: googol::Client =
    googol::Client::from_file("credentials.json", SCOPES);
#[cfg(not(debug_assertions))]
static GCP_AUTHENTICATION_CLIENT: googol::Client = googol::Client::from_environment(SCOPES);

async fn get_auth_token() -> Result<Token, Error> {
    GCP_AUTHENTICATION_CLIENT
        .get_token()
        .await
        .or(Err(Error::AuthError))
}
```

