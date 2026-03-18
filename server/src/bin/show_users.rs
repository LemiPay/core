use server::{establish_connection, models::User};
use diesel::prelude::*;

fn main() {
    use server::schema::user::dsl::user;

    let connection = &mut establish_connection();
    let results = user
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for u in results {
        println!("-----------\n");
        println!("{:?}", u.name);
        println!("{:?}", u.email);
    }
}