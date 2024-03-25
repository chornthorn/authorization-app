use authz_entity::User;
use authz_jwt::{encode_jwt, create_claims};

pub fn login() -> (String, String) {
    let create_claim = create_claims("user123".to_owned(), 60);
    let create_claim_refresh = create_claims("user123".to_owned(), 3600);
    let access_token = encode_jwt(create_claim);
    let refresh_token = encode_jwt(create_claim_refresh);

    (access_token, refresh_token)
}

pub fn get_all_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            username: "user1".to_owned(),
            password: "password1".to_owned(),
            email: "jonhdeo@gmaiol.com".to_owned(),
            first_name: Option::from("Jonh".to_owned()),
            last_name: Option::from("Deo".to_owned()),
            created_at: Default::default(),
            updated_at: Default::default(),
            deleted_at: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        let (access_token, refresh_token) = login();

        println!("Access Token: {}", access_token);

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
    }
}