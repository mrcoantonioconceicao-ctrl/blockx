use crate::domain::user::User;

pub trait UserRepository {
    fn save(&self, user: &User);

    fn find_by_email(
        &self,
        email: &str,
    ) -> Option<User>;
}
