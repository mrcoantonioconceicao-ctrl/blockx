mod api;
mod application;
mod bootstrap;
mod domain;
mod infrastructure;

fn main() {
    bootstrap::startup();
}
