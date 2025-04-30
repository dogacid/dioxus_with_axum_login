#[cfg(feature = "server")]
mod backend;

#[cfg(feature = "server")]
use dioxus::logger::tracing::log::{log, Level};

#[cfg(feature = "server")]
use crate::backend::DioxusAuthSession;

use dioxus::prelude::*;
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
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    use axum::Router;
    // Build a custom axum router
    let router = Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .layer(add_auth_layer())
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}


#[component]
fn App() -> Element {
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
                li { "user1 / 1234" }
                li { "user2 / 5678" }
            }
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
    rsx! {
        div {
            id: "main-page",
            h1 { "Main Page" }
            p { "This is the main page, that is protected by authentication." }
            Link {
                to: Route::Home {},
                "Back to Home"
            }
        }
    }
}

#[component]
fn UserStatus() -> Element {

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
    let mut auth = get_auth_session().await?;

    match auth.authenticate((username, password)).await {
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

fn use_user_context() -> Signal<Option<AppUser>> {
    try_use_context::<Signal<Option<AppUser>>>()
        .expect("User context not found. Ensure <App> is the root component.")
}
