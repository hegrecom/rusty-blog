use std::env;

pub fn database_pool() -> deadpool_diesel::postgres::Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Could not create database pool")
}
