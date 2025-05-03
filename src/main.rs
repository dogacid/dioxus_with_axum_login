#[cfg(feature = "server")]
mod backend;

#[cfg(feature = "server")]
use dioxus::logger::tracing::log::{log, Level};
#[cfg(feature = "server")]
use crate::backend::DioxusAuthSession;
#[cfg(feature = "server")]
use tokio::sync::OnceCell;
#[cfg(feature = "server")]
use sqlx::SqlitePool;

use dioxus::prelude::*;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/login")]
    LoginPage,
    #[layout(ProtectedRoute)]
        #[route("/main")]
        MainPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");


fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(launch_server());

    #[cfg(feature = "web")]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    use crate::backend::add_auth_layer;
    use sqlx::SqlitePool;

    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Initialize the database
    initialize_db().await;

    // Retrieve the database pool from the OnceCell
    let db = get_db();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    use axum::Router;
    // Build a custom axum router
    let router = Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .layer(add_auth_layer(db).await)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}


#[component]
fn App() -> Element {
    use dioxus::logger::tracing::{Level, info};
    use async_std::task::sleep;

    info!("App starting");

    use_context_provider(|| Signal::new(Option::<AppUser>::None));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn LoginPage() -> Element {
    let onsubmit = move |evt: FormEvent| async move {
        match login(evt.values()["username"].as_value(), evt.values()["password"].as_value()).await {
            Ok(_) => use_user_context().set(Some(AppUser { username: evt.values()["username"].as_value() })),
            Err(_) => use_user_context().set(None),
        }
    };

    use crate::dioxus_elements::textarea::autocomplete;
    rsx! {
        h1 { "Login" }
        form { onsubmit,
            input { r#type: "text", id: "username", name: "username", autocomplete: "username" }
            label { "Username" }
            br {}
            input { r#type: "password", id: "password", name: "password", autocomplete: "current-password" }
            label { "Password" }
            br {}
            button { "Login" }
        }
        Link {
            to: Route::Home {},
            "Back to Home"
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            id: "home",
            h1 { "Welcome!" }
            p { "This is a simple Dioxus webapp integrated with axum-login" }
            p { "There are two users preloaded in the backend:" }
            ul {
                li { "user1 / user1234" }
                li { "user2 / user2345" }
                li { "admin / admin1234" }
            }
            UserStatus {}
            Link {
                to: Route::MainPage {},
                "Main Page"
            }
        }
    }
}

#[component]
fn ProtectedRoute() -> Element {

    let user = use_user_context();

    if user.read().is_none() {
        let nav = navigator();
        // Redirect to login if unauthenticated
        rsx! {
            LoginPage {}
        }
    } else {
        rsx! {
            UserStatus {}
            Outlet::<Route> {}
        }
    }
}

#[component]
fn MainPage() -> Element {

    let mut user_items = use_signal(|| Vec::<String>::new());

    use_future(move || async move {
        match items().await {
            Ok(items) => user_items.set(items),
            Err(_) => {
                user_items.set(Vec::<String>::new())
            }
        }
    });

    rsx! {
        div {
            id: "main-page",
            h1 { "Main Page" }
            p { "This is the main page, that is protected by authentication." }
            div {
                h2 { "Items" }
                for item in user_items().iter() {
                    div { "{item}" }
                }
            }
            Link {
                to: Route::Home {},
                "Back to Home"
            }
        }
    }
}

#[component]
fn UserStatus() -> Element {

    // This code does a check with the server to see if the user is still logged in
    // Maybe, this is not needed, or  we switch JWT.
    use_future(move || async move {
        if let Ok(Some(user)) = current_user().await {
            if let Some(ref app_user) = *use_user_context().read() {
                if app_user.username != user.username {
                    use_user_context().set(None);
                }
            }
        } else {
            use_user_context().set(None);
        }
    });

    match *use_user_context().read() {
        Some(ref app_user) => rsx! {
            div {
                id: "user-status",
                h1 { "Welcome, {app_user.username}!" }
                button { onclick: move |_| async move { logout().await.unwrap(); use_user_context().set(None); }, "Logout" }
            }
        },
        None => rsx! {
            div {
                id: "user-status",
                h1 { "Not logged in" }
                // Link { to: Route::LoginPage {}, "Login" }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUser {
    pub username: String
}

#[cfg(feature = "server")]
pub async fn get_auth_session() -> Result<DioxusAuthSession, ServerFnError> {

    let context = server_context();
    let request_parts = context.request_parts();

    match request_parts.extensions.get::<DioxusAuthSession>() {
        Some(auth_session) => Ok(auth_session.clone()),
        None => Err(ServerFnError::ServerError("Auth session not found".to_string())),
    }
}

#[cfg(feature = "server")]
pub static DB: OnceCell<SqlitePool> = OnceCell::const_new();

#[cfg(feature = "server")]
pub async fn initialize_db() {
    let db = SqlitePool::connect(":memory:").await.unwrap();
    sqlx::migrate!().run(&db).await.unwrap();
    DB.set(db).expect("Failed to set the database pool");
}

#[cfg(feature = "server")]
pub fn get_db() -> &'static SqlitePool {
    DB.get().expect("Database pool not initialized")
}

#[server]
pub async fn current_user() -> Result<Option<AppUser>, ServerFnError> {
   let auth = get_auth_session().await?;

    match &auth.user {
        Some(user) => Ok(Some(AppUser { username: user.username.clone() })),
        None => Ok(None),
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::backend::Credentials;
    let mut auth = get_auth_session().await?;

    match auth.authenticate(Credentials { username: username, password: password, next: None }).await {
        Ok(Some(user)) => {
            match &auth.login(&user).await {
                Ok(_) => {
                    log!(Level::Info, "Login succeeded for {}", user.username);
                    Ok(())
                }
                Err(_) => {
                    log!(Level::Info, "Login failed");
                    Err(ServerFnError::ServerError("Login failed".to_string()))
                }
            }
        },
        Ok(None) => {
            log!(Level::Info, "Login failed - None User match");
            Err(ServerFnError::ServerError("No User".to_string()))
        },
        Err(_) => Err(ServerFnError::ServerError("Error occured".to_string())),
    }

}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let mut auth = get_auth_session().await?;
    auth.logout().await?;
    Ok(())
}

// #[server]
// pub async fn items() -> Result<Vec<String>, ServerFnError> {
//     use std::collections::HashMap;
//     let auth = get_auth_session().await?;
//
//     println!("start");
//
//     let mut map: HashMap<String, Vec<String>> = HashMap::new();
//
//     // Add a key with a vector of strings
//     map.insert(
//         "user1".to_string(),
//         vec!["item1".to_string(), "item2".to_string()],
//     );
//
//     // Add another key
//     map.insert(
//         "user2".to_string(),
//         vec!["item3".to_string(), "item4".to_string()],
//     );
//
//     match &auth.user {
//         Some(user) => {
//             if let Some(values) = map.get(&user.username) {
//                 Ok(values.to_vec())
//             } else {
//                 Err(ServerFnError::ServerError("No user".to_string()))
//             }
//         },
//         None => Err(ServerFnError::ServerError("No user".to_string())),
//     }
//
//
// }

#[server]
pub async fn items() -> Result<Vec<String>, ServerFnError> {
    use sqlx::SqlitePool;
    use std::convert::Infallible;

    println!("start");

    let auth = get_auth_session().await?;
    println!("have auth");

    let db = get_db();  

    println!("have db");

    match &auth.user {
        Some(user) => {
            let items = sqlx::query_scalar("SELECT name FROM items WHERE originator = ?")
                .bind(&user.id)
                .fetch_all(db)
                .await?;
            println!("Items in successfully");
            Ok(items)
        }
        None => Err(ServerFnError::ServerError("No user in session".to_string())),
    }
}

fn use_user_context() -> Signal<Option<AppUser>> {
    try_use_context::<Signal<Option<AppUser>>>()
        .expect("User context not found. Ensure <App> is the root component.")
}
