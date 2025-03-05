// use rand_core::OsRng;

// use argon2::{
//     password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
//     Argon2,
// };

pub fn hash_pass(_salt: String, password: String) -> String {
    // let password = password.as_bytes();
    // let salt = SaltString::generate(&mut OsRng);

    // let argon2 = Argon2::default();
    // let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    // let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    // assert!(Argon2::default()
    //     .verify_password(password, &parsed_hash)
    //     .is_ok());

    // password_hash
    password
}
