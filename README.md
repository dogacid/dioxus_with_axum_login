# Example of Dioxus integrated with axum-login for authentication

This project is a snapshot of some early work I did to integrate dioxus fullstack web app with the [axum-login crate](https://github.com/maxcountryman/axum-login).
axum-login is a authentication library for axum. It provides a way to handle user authentication and session management in a straightforward manner.
It is only an in memory version with default users:

---
    - username: user1
    - password: user1234
---
    - username: user2
    - password: user1234
---
    - username: admin
    - password: admin1234
---

argon2 is used for hashing, and there is some example code for protecting routes. This is very early code, and part of my learning exercise.

### Todos

- Current this is session based  authentication
- I have not yet implemented JWT authentication, but I plan to do so in the future.
- Intro SeaORM so the server functions can work with more model like application code

Productive feedback welcomed, as I haven't spent enough time reading into the dioxus apis as yet.

### Getting Started

As we are now using a sqlite backend that is file based, you need to create a database file before running the server. 
It is probably easiest to install sqlx cli and run this command. Database migrations are currently automatically done on start up by the server.

You can do this by running the following command from the root of the project:
```bash
 sqlx database create --database-url sqlite://development.db
```

This is the name of the database file found in the `.env` file. You can change this to whatever you like, but make sure to update the `.env` file accordingly.

To start up:
```bash
dx serve
```

