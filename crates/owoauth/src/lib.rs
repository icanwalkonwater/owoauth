pub mod authorization_server;
mod types;

pub use endpoints::authorization::*;
pub use types::*;

pub(crate) mod endpoints {
    pub(crate) mod authorization {
        use crate::*;

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub struct AuthorizationRequest<SCOPE = String, STATE = String> {
            pub response_type: ResponseType,
            pub client_id: ClientId,
            pub redirect_uri: Option<RedirectUri>,
            pub scope: Option<SCOPE>,
            pub state: Option<STATE>,
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case", untagged)]
        pub enum AuthorizationResponse {
            Success {
                code: String,
                state: Option<String>,
            },
            Failure {
                error: AuthorizationResponseErrorCode,
                error_description: Option<String>,
                error_uri: Option<String>,
                state: Option<String>,
            },
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum AuthorizationResponseErrorCode {
            InvalidRequest,
            UnauthorizedClient,
            AccessClient,
            UnsupportedResponseType,
            InvalidScope,
            ServerError,
            TemporarilyUnavailable,
        }
    }

    pub(crate) mod token {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub struct AccessTokenRequest {
            pub grant_type: String,
            pub code: String,
            pub redirect_uri: Option<String>,
            pub client_id: Option<String>,
        }
    }
}
