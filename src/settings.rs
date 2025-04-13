use axum_keycloak_auth::PassthroughMode;
use serde::Deserialize;

/// Configuration settings for Keycloak authentication.
///
/// This struct should be placed under the `settings.keycloak_settings` section
/// of your Loco application's `config/config.yaml`. It provides all values
/// needed to initialize the Keycloak authentication layer.
#[derive(Clone, Deserialize)]
pub struct KeycloakSettings {
    /// The full URL to your Keycloak server (e.g. `https://sso.example.com`).
    pub url: String,
    /// The realm name in Keycloak (e.g. `myrealm`).
    pub realm: String,
    /// A list of expected audiences in the token (typically contains `"account"`).
    pub expected_audiences: Vec<String>,
    /// The mode that determines how the authentication layer behaves.
    ///
    /// - `PassthroughMode::Block`: Return `401 Unauthorized` on authentication failure.
    /// - `PassthroughMode::Pass`: Allow unauthenticated access and set auth status as an extension.
    ///
    /// Default: `Block`
    pub passthrough_mode: PassthroughModeDef,
    /// Whether to persist raw Keycloak claims as an Axum extension.
    ///
    /// Set this to `true` if you want access to the raw token contents.
    pub persist_raw_claims: bool,
}

/// Root struct to hold all custom application settings.
///
/// This is typically deserialized from the `settings:` section in Loco's
/// `config/config.yaml`.
/// ## Sample configuration
/// ```yaml
/// settings:
///  keycloak_settings:
///  url: "https://sso.example.com"
///  realm: "myrealm"
///  expected_audiences:
///    - "account"
///  passthrough_mode: "Block"  # or "Pass"
///  persist_raw_claims: false
/// ```
#[derive(Clone, Deserialize)]
pub struct Settings {
    pub keycloak_settings: KeycloakSettings,
}

#[derive(Debug, Clone)]
pub struct PassthroughModeDef(pub PassthroughMode);

impl<'de> Deserialize<'de> for PassthroughModeDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "block" => Ok(PassthroughModeDef(PassthroughMode::Block)),
            "pass" => Ok(PassthroughModeDef(PassthroughMode::Pass)),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid passthrough mode: {}",
                s
            ))),
        }
    }
}
