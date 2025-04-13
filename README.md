# 🔐 loco-keycloak-auth

A plug-and-play Keycloak authentication layer for [Loco.rs](https://github.com/loco-rs/loco), powered by [axum-keycloak-auth](https://crates.io/crates/axum-keycloak-auth).  
This crate allows you to easily add secure Keycloak authentication to your Loco web app, with full control over protected routes and clean YAML-based config.

---

## ✨ Features

- ✅ Simple integration with Loco initializers
- ✅ Based on `axum-keycloak-auth`
- ✅ Configurable via `config.yaml`
- ✅ Supports `Block` and `Pass` passthrough modes
- ✅ Designed to be flexible: apply middleware only where you want it
- ✅ Ideal for securing internal APIs or user-facing endpoints

---

## ⚙️ Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
loco-keycloak-auth = { git = "https://github.com/yourname/loco-keycloak-auth" }
```

> **Note**: If you’re using a local path for development:

```toml
loco-keycloak-auth = { path = "../loco-keycloak-auth" }
```

---

## 🛠 Setup

### 1. Add Keycloak config to your `config/config.yaml`

```yaml
settings:
  keycloak_settings:
    url: "https://keycloak.example.com"
    realm: "myrealm"
    expected_audiences:
      - "account"
    passthrough_mode: "Block" # or "Pass"
    persist_raw_claims: false
```

### 2. Add the initializer to your `App` in `app.rs` if you want to have all routes protected.

```rust
use loco_keycloak_auth::KeycloakAuthInitializer;

#[async_trait]
impl Hooks for App {
    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        let keycloak_auth = loco_keycloak_auth::initializer::KeycloakAuthInitializer {};
        Ok(vec![Box::new(keycloak_auth)])
    }
}
```

---

## 🔒 Usage

### Protect specific endpoints

```rust
use loco_keycloak_auth::Keycloak;

fn routes(ctx: &AppContext) -> Routes {
    let keycloak = Keycloak::from_context(ctx).expect("Failed to create Keycloak layer");

    Routes::new()
        .prefix("secure")
        .add("/profile", get(profile_handler).layer(keycloak.layer))
}
```

---

## 📦 API

### Settings struct

```rust
pub struct KeycloakSettings {
    pub url: String,
    pub realm: String,
    pub expected_audiences: Vec<String>,
    pub passthrough_mode: PassthroughMode, // "Block" or "Pass"
    pub persist_raw_claims: bool,
}
```

> `PassthroughMode` lets you decide whether unauthenticated requests should be blocked or passed along.

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### 1. Clone and link locally

```bash
git clone https://github.com/GKaszewski/loco-keycloak-auth
cd loco-keycloak-auth
```

### 2. Use in your Loco project with a local path

```toml
[dependencies]
loco-keycloak-auth = { path = "../loco-keycloak-auth" }
```

### 3. Run tests if there are any

```bash
cargo test
```

### 4. Submit a PR 🚀

Please open an issue or discussion first for larger feature proposals or breaking changes.

---

## 📄 License

MIT

---

## 🙌 Credits

- Built with ❤️ for the [Loco.rs](https://github.com/loco-rs/loco) ecosystem
- Powered by [axum-keycloak-auth](https://github.com/filiptibell/axum-keycloak-auth)

---

## 📫 Contact

Questions? Ideas? Want to contribute together?  
Open an issue or reach out on [GitHub Discussions](https://github.com/GKaszewski/loco-keycloak-auth/discussions).
