use crate::domain::user::User;

pub trait UserRepository {
    fn save(&self, user: &User);
}
