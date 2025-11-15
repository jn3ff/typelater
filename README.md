# Typelater

Foundational types and their children make me want to type later

## Status/Version

Unpublished

## Justification

This crate contains a very simple derive macro for when you want to easily convert between types, subject to one type being a logical field-transform of another. For example, consider the simple example between User and PublicUser.

```rust
struct User {
    username: String,
    email: String,
    last_login: chrono::DateTime<chrono::Utc>
}

struct PublicUser {
    username: String,
}
```

One could argue you'd simply enforce this DTO-like notion at application boundary, with serialization attributes, e.g.

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    username: String,
    #[serde(skip_serializing)]
    email: String,
    #[serde(skip_serializing)]
    last_login: chrono::DateTime<chrono::Utc>
}
```

This is fine and good, and often correct, but sometimes we have varied simultaneous relationships. Maybe an email service needs to receive the username + email combination (along with whatever other metadata), while 3rd party consumers are allowed APIs that don't expose email. Tracking and/or implementing different serialization relationships is (in this crate's opinion) sometimes more overhead than specifying different types for different contracts and letting the handy macros do the heavy lifting.

So, we have a plausible reason for wanting to transform User into PublicUser. How are we going to do this idiomatically? We'd write a From<T> impl:

```rust
impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            username: value.username
        }
    }
}
```

Super easy. And tedious! Especially if we have multiple view structs for various contracts. Typelater generates the same code, but with a simpler api (especially when there are more than 3 fields).

## Example

```rust
use typelater::Typelater;

struct User {
    username: String,
    email: String,
    last_login: chrono::DateTime<chrono::Utc>
}

#[derive(Typelater)]
#[typelater(from = "User")]
struct PublicUser {
    username: String,
}
```

Typelater supports aliasing fields, like:

```rust
use typelater::Typelater;

struct User {
    username: String,
    email: String,
    last_login: chrono::DateTime<chrono::Utc>
}

#[derive(Typelater)]
#[typelater(from = "User")]
struct PublicUser {
    #[typelater(alias = "username")]
    name: String,
}
```

Typelater supports nested fields, like:

```rust
use typelater::Typelater;

struct UserMetadata {
    followers: usize,
    last_login: chrono::DateTime<chrono::Utc>,
}

struct User {
    username: String,
    email: String,
    metadata: UserMetadata,
}

#[derive(Typelater)]
#[typelater(from = "User")]
struct PublicUser {
    username: String,
    #[typelater(path = "metadata.followers")]
    followers: usize,
}
```

The need for this crate is dubious, but I wanted to learn more about proc macros and this is the lens I have chosen to do it under.

TODO:

- [ ] Add generate option when the "child" has overflowing fields (e.g. we need to generate display fields in DTOs)
- [ ] Add default option when the "child" has overflowing fields (e.g. we're gonna do something else with this value now)
