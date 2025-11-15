use typelater::Typelater;

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    last_login: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Typelater)]
#[typelater(from = "User")]
struct PublicUser {
    #[typelater(alias = "username")]
    name: String,
}

#[test]
fn converts_user_with_alias_into_public_user() {
    let user = User {
        username: "Jerry".to_string(),
        email: String::new(),
        last_login: chrono::DateTime::<chrono::Utc>::MIN_UTC,
    };
    let public_user: PublicUser = user.into();
    assert_eq!(public_user.name, "Jerry");
}
