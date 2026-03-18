use server::*;
use std::io::{stdin};

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

    let user = create_post(connection, name, email, password);
    println!("\nWelcome {name} with id {}", user.id);
}