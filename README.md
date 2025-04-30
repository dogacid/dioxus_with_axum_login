# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

This project is a snapshot of some early work I did to integrate dioxus fullstack web app with the [axum-login crate](https://github.com/maxcountryman/axum-login).
axum-login is a authentication library for axum. It provides a way to handle user authentication and session management in a straightforward manner.
It is only an in Memory version with default users:
---
    - username: user1
    - password: 1234
---
    - username: user2
    - password: 5678
---
argon2 is used for hashing, and there is some example code for protecting routes. This is very early code, and part of my learning exercise.

### Todos

- Will need to add a background auth checker for the front-end
- Fix Bug: When refresh page lose the logged in user in the front-end, when back-end still has session, the background checker might fix that.
- Plan to add sqlx / sqlite database, for user database and session management.

Any suggestions will be welcomed, as I ev.

