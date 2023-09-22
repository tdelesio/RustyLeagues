use std::env;
use dotenv::dotenv;

fn main() {

    let key = "DATABASE_URL";
    println!("Database URL={}",env::var(key).expect("Error: DATABASE_URL not found"));
    // api::main();

    dotenv().ok();
    // dotenv().expect("Failed to read .env file");

    println!("Database URL={}",env::var("DB_URL").expect("Error: DATABASE_URL not found"));

    // dotenv().ok();
    //
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
}
