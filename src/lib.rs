pub mod initializer;
pub mod settings;

pub use axum_keycloak_auth::decode::KeycloakToken;
use axum_keycloak_auth::{
    Url,
    instance::{KeycloakAuthInstance, KeycloakConfig},
    layer::KeycloakAuthLayer,
};
use loco_rs::prelude::*;
use settings::{KeycloakSettings, Settings};

/// Keycloak is a struct that holds the Keycloak authentication layer.
/// ## Usage
/// ```rust
/// let keycloak = Keycloak::from_context(ctx).expect("Failed to create Keycloak layer");
/// let router = Router::new()
///     .route("/protected", get(protected_handler))
///    .layer(keycloak.layer);
/// ```
pub struct Keycloak {
    pub layer: KeycloakAuthLayer<String>,
}

impl Keycloak {
    pub fn from_context(ctx: &AppContext) -> Result<Self> {
        build_keycloak_layer(ctx).map(|layer| Keycloak { layer })
    }
}

/// This function builds the Keycloak authentication layer using the settings
/// from the application context.
pub fn build_keycloak_layer(ctx: &AppContext) -> Result<KeycloakAuthLayer<String>> {
    let full_settings: Settings = serde_json::from_value(
        ctx.config
            .settings
            .clone()
            .ok_or_else(|| Error::Message("Missing `settings` in config".into()))?,
    )
    .map_err(|err| Error::Message(format!("Invalid settings: {}", err)))?;

    let settings: KeycloakSettings = full_settings.keycloak_settings;

    let instance =
        KeycloakAuthInstance::new(
            KeycloakConfig::builder()
                .server(Url::parse(&settings.url).map_err(|err| {
                    Error::Message(format!("Invalid Keycloak server URL: {}", err))
                })?)
                .realm(settings.realm)
                .build(),
        );

    Ok(KeycloakAuthLayer::<String>::builder()
        .instance(instance)
        .passthrough_mode(settings.passthrough_mode.0)
        .persist_raw_claims(settings.persist_raw_claims)
        .expected_audiences(settings.expected_audiences)
        .build())
}
