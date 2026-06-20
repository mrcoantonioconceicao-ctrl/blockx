use shared::time::now;

use crate::api;

use crate::application::{
    create_user,
    login_user,
};

use crate::infrastructure::{
    in_memory_user_repository::InMemoryUserRepository,
    user_repository::UserRepository,
};

pub fn startup() {

    println!("=================================");
    println!("BlockX Auth Service");
    println!("Started at: {}", now());
    println!("=================================");

    api::register_routes();

    let repository =
        InMemoryUserRepository::new();

    let user =
        create_user::execute(
            &repository,
            "admin@blockx.io".to_string(),
            "123456".to_string(),
        )
        .unwrap();

    repository.save(&user);

    let token =
        login_user::execute(
            &repository,
            "admin@blockx.io",
            "123456",
        );

    println!(
        "JWT: {:?}",
        token
    );

    println!(
        "User created: {:?}",
        user
    );
}
