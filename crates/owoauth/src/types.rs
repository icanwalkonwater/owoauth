use std::collections::HashSet;

use serde::Serialize;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct ClientId<T = uuid::Uuid>(pub(crate) T);

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct RedirectUri(String);

impl From<String> for RedirectUri {
    fn from(uri: String) -> Self {
        Self(uri)
    }
}

impl RedirectUri {
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_redirect_location(
        &self,
        _res: AuthorizationResponse,
        _additional_qs: impl Serialize,
    ) -> String {
        todo!()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ResponseType {
    Code,
    Token,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GrantType {
    AuthorizationCode,
    Implicit,
    Password,
    ClientCredentials,
    RefreshToken,
    #[serde(rename = "urn:ietf:params:oauth:grant-type:jwt-bearer")]
    JwtBearer,
    #[serde(rename = "urn:ietf:params:oauth:grant-type:saml2-bearer")]
    Saml2Bearer,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ClientAuthMethod {
    None,
    ClientSecretPost,
    ClientSecretBasic,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Scope(String);

#[derive(Default)]
pub struct ClientMetadata {
    pub redirect_uris: HashSet<RedirectUri>,
    pub token_endpoint_auth_method: Option<ClientAuthMethod>,
    pub grant_types: HashSet<GrantType>,
    pub response_types: HashSet<ResponseType>,
    pub client_name: String,
    pub client_uri: String,
    pub logo_uri: String,
    pub scope: Vec<Scope>,
    pub contacts: Vec<String>,
    pub tos_uri: Option<String>,
    pub policy_uri: Option<String>,
    pub jwks: Option<ClientMetadataJwks>,
    pub software_id: Option<String>,
    pub software_version: Option<String>,
}

pub enum ClientMetadataJwks {
    Inline(String),
    Uri(String),
}

// other stuff

pub struct AuthorizationRequest {
    pub response_type: Option<ResponseType>,
    pub client_id: Option<ClientId>,
    pub redirect_uri: Option<RedirectUri>,
    pub scope: Option<String>,
    pub state: Option<String>,
}

pub struct AuthorizationResponse {
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code: Option<String>,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<String>,
}

pub struct TokenRequest {
    pub client_id: Option<ClientId>,
    pub client_secret: Option<String>,
    pub grant_type: Option<String>,
    pub redirect_uri: Option<String>,
    pub scope: Option<String>,
    pub code: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<String>,
}

pub struct TokenResponse {
    pub scope: Option<String>,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<String>,
    pub refresh_token: Option<String>,
}
