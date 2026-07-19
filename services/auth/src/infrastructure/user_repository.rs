use crate::domain::user::User;

pub trait UserRepository {
    fn save(&self, user: &User);

    fn find_by_email(&self, email: &str) -> Option<User>;

    fn exists(&self, email: &str) -> bool {
        self.find_by_email(email).is_some()
    }
}
