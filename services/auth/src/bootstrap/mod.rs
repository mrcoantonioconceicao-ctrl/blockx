use shared::time::now;

use crate::api;
use crate::application::create_user;
use crate::application::login_user;
use crate::infrastructure::in_memory_user_repository::InMemoryUserRepository;
use crate::infrastructure::user_repository::UserRepository;

pub fn startup() {
    println!("=================================");
    println!("BlockX Auth Service");
    println!("Started at: {}", now());
    println!("=================================");

    api::register_routes();

    let repository = InMemoryUserRepository::new();

    let user = create_user::execute(
        "admin@blockx.io".to_string(),
        "123456".to_string(),
    );

    repository.save(&user);

    let token = login_user::execute(
        &user,
        "123456",
    );

    println!(
        "Authentication result: {:?}",
        token
    );

    println!(
        "User created: {:?}",
        user
    );
}
