use std::{collections::HashMap, net::Ipv6Addr, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use owoauth::{
    authorization_server::{errors::AuthorizationEndpointError, AuthorizationServer},
    AuthorizationResponse, RedirectUri,
};
use tower_http::trace::TraceLayer;
use tracing::Level;

async fn hello_world() -> impl IntoResponse {
    "Hello world"
}

#[derive(Clone)]
struct ExampleAppState {
    oauth_authorization_server: Arc<AuthorizationServer>,
}

#[derive(serde::Deserialize)]
pub struct AsAuthorizationRequest {
    #[serde(flatten)]
    pub request: owoauth::AuthorizationRequest,
    #[serde(flatten)]
    pub passthrough: HashMap<String, String>,
}

async fn oauth_as_authorization(
    State(state): State<ExampleAppState>,
    Query(params): Query<AsAuthorizationRequest>,
) -> Response {
    // The authorization server validates the request to ensure that all
    // required parameters are present and valid.  If the request is valid,
    // the authorization server authenticates the resource owner and obtains
    // an authorization decision (by asking the resource owner or by
    // establishing approval via other means).

    //  When a decision is established, the authorization server directs the
    //  user-agent to the provided client redirection URI using an HTTP
    //  redirection response, or by other means available to it via the
    //  user-agent.

    let request = oauth_as.authorization_endpoint().validate_request(request) else {
        return "something fishy is going on";
    };

    match request {
        Authorized {
            redirect_uri,
            response_params,
        } => todo!(),
        RequiresAdditionalConsent {
            client_info,
            wanted_scopes,
            original_request,
        } => todo!(),
    }

    todo!()
}

async fn oauth_as_token() -> Response {
    // The authorization server MUST:
    //
    // o  require client authentication for confidential clients or for any
    //    client that was issued client credentials (or with other
    //    authentication requirements),
    //
    // o  authenticate the client if client authentication is included,
    //
    // o  ensure that the authorization code was issued to the authenticated
    //    confidential client, or if the client is public, ensure that the
    //    code was issued to "client_id" in the request,
    //
    // o  verify that the authorization code is valid, and
    //
    // o  ensure that the "redirect_uri" parameter is present if the
    //    "redirect_uri" parameter was included in the initial authorization
    //    request as described in Section 4.1.1, and if included ensure that
    //    their values are identical.
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app_state = ExampleAppState {
        oauth_authorization_server: Arc::new(AuthorizationServer {}),
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/oauth/as/authorization", get(oauth_as_authorization))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    axum::Server::bind(&(Ipv6Addr::UNSPECIFIED, 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
