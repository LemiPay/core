use diesel::prelude::*;
use server::data::database::{establish_connection};
use server::models::User;

fn get_users(conn: &mut PgConnection, limit: i64) -> Vec<User> {
    use server::schema::user::dsl::user;

    user
        .limit(limit)
        .select(User::as_select())
        .load(conn)
        .expect("Error loading posts")
}

fn main() {
    let connection = &mut establish_connection();

    let results = get_users(connection, 5);

    println!("Displaying {} posts", results.len());

    for u in results {
        println!("-----------\n");
        println!("{:?}", u.name);
        println!("{:?}", u.email);
    }
}