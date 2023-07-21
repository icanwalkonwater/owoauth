use crate::ClientMetadata;

#[async_trait::async_trait]
pub trait AuthorizationServerBackend {}

pub struct AuthorizationServer<BACKEND> {
    backend: BACKEND,
}

impl<B> AuthorizationServer<B>
where
    B: AuthorizationServerBackend,
{
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn register_client(&self, metadata: ClientMetadata) {
        todo!()
    }
}

pub mod errors {
    #[derive(Debug, snafu::Snafu)]
    pub enum AuthorizationEndpointError {
        InvalidRedirectionUri,
        InvalidClientId,
    }
}

mod tests {
    use std::collections::HashSet;

    use crate::{ClientMetadata, RedirectUri};

    use super::{AuthorizationServer, AuthorizationServerBackend};

    struct MockAsBackend;

    impl AuthorizationServerBackend for MockAsBackend {}

    #[test]
    fn client_register() {
        let auth_server = AuthorizationServer::new(MockAsBackend);

        auth_server.register_client(ClientMetadata {
            redirect_uris: HashSet::from_iter(std::iter::once(RedirectUri::from(
                "oui".to_string(),
            ))),
            ..Default::default()
        });
    }
}
