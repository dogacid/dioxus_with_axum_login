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

- Will need to add a background auth checker for the front-end
  - This is has been added, need further testing.
  - Ensure Front-End in sync with Back-end, maybe look into axum websocket integration.

- Plan to add sqlx / sqlite database, for user database and session management.

Productive feedback welcomed, as I haven't spent enough time reading into the dioxus apis as yet.

To run:

```bash
dx serve
```

