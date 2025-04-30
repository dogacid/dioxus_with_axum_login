use std::collections::HashMap;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder, AuthSession, AuthUser, AuthnBackend, UserId};
use async_trait::async_trait;
use argon2::password_hash::{PasswordVerifier, SaltString};
use argon2::password_hash::rand_core::OsRng;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::tower_sessions::cookie::time::Duration;
use serde::{Serialize, Deserialize};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash.as_bytes()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Backend {
    pub users: HashMap<String, User>,
}

impl Backend {
    pub fn new(users: HashMap<String, User>) -> Self {
        Backend { users }
    }
}

fn create_backend() -> Backend {
    let mut users = HashMap::new();

    users.insert(
        "user1".to_string(),
        User {
            username: "user1".to_string(),
            password_hash: hash_password("1234"), // Replace with actual password hash
        },
    );

    users.insert(
        "user2".to_string(),
        User {
            username: "user2".to_string(),
            password_hash: hash_password("5678"), // Replace with actual password hash
        },
    );

    Backend::new(users)

}

pub type Credentials = (String, String);

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        ( username, password ): Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if let Some(user) = self.users.get(&username) {
            if verify_password(&password, &user.password_hash) {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub type DioxusAuthSession = AuthSession<Backend>;


pub fn add_auth_layer() -> AuthManagerLayer<Backend, MemoryStore> {

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

    // Auth service.
    let backend = create_backend();
    AuthManagerLayerBuilder::new(backend, session_layer).build()

}
