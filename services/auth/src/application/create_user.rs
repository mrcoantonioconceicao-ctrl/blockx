use shared::ids::generate_id;

use crate::application::password_service::hash_password;
use crate::domain::user::User;

pub fn execute(
    email: String,
    password: String,
) -> User {
    let password_hash = hash_password(&password);

    let user = User {
        id: generate_id(),
        email,
        password_hash,
    };

    user
}
