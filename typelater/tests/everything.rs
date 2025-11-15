use typelater::Typelater;

#[allow(dead_code)]
struct UserMetadata {
    followers: usize,
    last_login: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    metadata: UserMetadata,
}

#[derive(Typelater)]
#[typelater(from = "User")]
struct PublicUser {
    #[typelater(alias = "username")]
    name: String,
    #[typelater(path = "metadata.followers")]
    followers: usize,
}

#[test]
fn converts_user_with_nested_field_and_alias_into_public_user() {
    let user = User {
        username: "Jerry".to_string(),
        email: String::new(),
        metadata: UserMetadata {
            followers: 2,
            last_login: chrono::DateTime::<chrono::Utc>::MIN_UTC,
        },
    };
    let public_user: PublicUser = user.into();
    assert_eq!(public_user.name, "Jerry");
    assert_eq!(public_user.followers, 2);
}
