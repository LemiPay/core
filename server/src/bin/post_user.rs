use std::io::{stdin};

use diesel::{
    PgConnection,
    RunQueryDsl, // get result
    SelectableHelper // as_returning
};

use server::data::database::{establish_connection};
use server::models::{NewUser, User};

pub fn create_user(conn: &mut PgConnection, name: &str, email: &str, password: &str) -> User {
    use server::schema::user;

    let new_user = NewUser { name, email, password };

    diesel::insert_into(user::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}



fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut email = String::new();
    let mut password = String::new();

    println!("What would you like your name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    println!("\nOk! And what is your email {name}?\n",);
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    println!("\nPassword? {name} - {email}?\n",);
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();

    let user = create_user(connection, name, email, password);
    println!("\nWelcome {name} with id {}", user.id);
}

