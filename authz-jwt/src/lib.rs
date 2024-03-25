use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{TimeDelta, Utc};
use authz_dto::Claims;


/// Loads RSA private and public keys from file.
///
/// # Returns
/// A tuple with two `Vec<u8>` values: the private key and the public key.
///
fn load_rsa_keys() -> (Vec<u8>, Vec<u8>) {
    let private_key = include_bytes!("../../keys/private_key.pem");
    let public_key = include_bytes!("../../keys/public_key.pem");
    (private_key.to_vec(), public_key.to_vec())
}

/// Create claims for JWT token.
///
/// # Arguments
///
/// * `sub` - The subject of the token (required).
///
/// # Returns
///
/// A `Claims` struct containing the claims for the token.
///
/// # Examples
///
/// ```rust
/// use authz_jwt::create_claims;
///
/// let claims = create_claims("user123".to_owned(),60);
///
/// assert_eq!(claims.sub, "user123");
/// assert!(claims.iat > 0);
/// assert!(claims.exp > claims.iat);
/// ```
pub fn create_claims(sub: String,exp: i64) -> Claims {
    let now_timestamp = Utc::now();
    let expire_timestamp = now_timestamp + TimeDelta::try_minutes(exp).unwrap();

    Claims {
        sub: sub.to_owned(),
        iat: now_timestamp.timestamp() as usize,
        exp: expire_timestamp.timestamp() as usize,
    }
}

/// Encode a JSON Web Token (JWT) with the given claims.
///
/// # Arguments
///
/// * `claims` - A reference to the claims for the JWT.
///
/// # Returns
///
/// The encoded JWT as a string.
///
/// # Panics
///
/// This function will panic if loading the RSA keys or encoding the JWT fails.
///
/// # Examples
///
/// ```
/// use authz_jwt::encode_jwt;
/// use authz_dto::Claims;
///
/// let claims = Claims {
///     sub: "user123".to_owned(),
///     iat: 1516239022,
///     exp: 1516239022,
/// };
/// let encoded_jwt = encode_jwt(claims);
/// println!("Encoded JWT: {}", encoded_jwt);
/// ```
pub fn encode_jwt(claims: Claims) -> String {
    let header = Header::new(Algorithm::RS512);
    let (private_key, _) = load_rsa_keys();
    let encoding_key = EncodingKey::from_rsa_pem(&private_key).unwrap();
    encode(&header, &claims, &encoding_key).unwrap()
}

/// Decode a JSON Web Token (JWT) and extract the claims.
///
/// # Arguments
///
/// * `token` - A string representing the JWT.
///
/// # Panics
///
/// This function panics if any of the following conditions occur:
///
/// * The RSA key pair cannot be loaded.
/// * The provided token cannot be decoded.
/// * The token validation fails.
///
/// # Examples
///
/// ```rust
/// use authz_dto::Claims;
/// use authz_jwt::decode_jwt;
///
/// let jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.IxPdtyY3nfGH-PDIKAEn3Wek";
/// let claims = decode_jwt(jwt);
/// println!("Claims: {:?}", claims);
pub fn decode_jwt(token: &str) -> Claims {
    let (_, public_key) = load_rsa_keys();
    let decoding_key = DecodingKey::from_rsa_pem(&public_key).unwrap();
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::RS512)).unwrap();
    token_data.claims
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let claims = create_claims("user123".to_owned(),60);
        let token = encode_jwt(claims);
        let decoded_claims = decode_jwt(&token);
        assert_eq!(claims.sub, decoded_claims.sub);
    }
}