use crate::application::create_user;
use crate::application::login_user;
use crate::domain::user::User;
use crate::infrastructure::in_memory_user_repository::InMemoryUserRepository;
use crate::infrastructure::user_repository::UserRepository;

pub fn run(repository: InMemoryUserRepository) {

    // 1. Criar usuário
    let user: User = create_user::execute(
        "admin@blockx.io".to_string(),
        "123456".to_string(),
    ).expect("create user failed");

    // 2. Persistir usuário
    UserRepository::save(&repository, &user);

    // 3. Login
    let token = login_user::execute(&user)
        .expect("login failed");

    println!("TOKEN: {}", token);
}
