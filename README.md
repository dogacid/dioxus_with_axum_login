# Example of Dioxus integrated with axum-login for authentication

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

Any suggestions will be welcomed.

To run:

```bash
dx serve
```

