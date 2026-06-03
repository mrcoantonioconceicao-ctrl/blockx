use shared::ids::generate_id;

use crate::domain::user::User;

pub fn execute(email: String) -> User {
    let user = User {
        id: generate_id(),
        email,
    };

    user
}
