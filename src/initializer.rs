use axum::Router;
use loco_rs::prelude::*;

use crate::Keycloak;

/// KeycloakAuthInitializer is an initializer for the Keycloak authentication layer.
/// Use this if you want to add Keycloak authentication to all routes in your application.
/// It will automatically read the Keycloak settings from the application context.
/// This initializer is typically used in the `app.rs` file of your Loco application.
/// If you want to have more control over the Keycloak layer, you can use the `Keycloak` struct
/// and add the layer to the routes directly.
/// ## Example
/// ```rust
/// async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
///     let keycloak_auth = loco_keycloak_auth::initializer::KeycloakAuthInitializer {};
///     Ok(vec![Box::new(keycloak_auth)])
/// }
/// ```
pub struct KeycloakAuthInitializer;

#[async_trait]
impl Initializer for KeycloakAuthInitializer {
    fn name(&self) -> String {
        "keycloak_auth".to_string()
    }

    async fn after_routes(&self, router: Router, ctx: &AppContext) -> Result<Router> {
        let keycloak = Keycloak::from_context(ctx)
            .map_err(|err| Error::Message(format!("Failed to create Keycloak layer: {}", err)))?;
        Ok(router.layer(keycloak.layer))
    }
}
