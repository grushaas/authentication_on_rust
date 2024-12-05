extern crate ring;
extern crate data_encoding;

use pass_work::verify_password;
use postgres::{Client, Error, NoTls};
use user::{findUser, User};
use std::io;

mod pass_work;
mod actions_with_db;
mod user;


fn authentication() -> user::User{
    let mut login = String::new();
    let mut password = String::new();
    let mut email = String::new();
    println!("Введите логин: ");
    let _ = io::stdin().read_line(&mut login);
    println!();
    println!("Введите пароль: ");
    let _ = io::stdin().read_line(&mut password);
    let pass_hash;
    match pass_work::hash_password(&password) {
        Ok(ph) => pass_hash = ph,
        Err(_e) => pass_hash = "incorrect".to_string(),
    }

    println!();
    println!("Введите почту: ");
    let _ = io::stdin().read_line(&mut email);

    let people = user::User {
        login,
        password: pass_hash,
        email
    };
    people
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=grusha password=MaxFil2005 dbname=authentication", NoTls)?;
    let mut user;

    let mut choice= -1;

    while choice != 0 {
        println!("Авторизация");
        println!("1. Регистрация");
        println!("2. Вход");
        println!("0. Выход");

        let mut input = String::new();
        println!("Введите действие:");

        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        choice = input.trim().parse().expect("Введите корректное целое число");
        
        match choice {
            1 => {
                user = authentication();
                match actions_with_db::db::Insert(&mut client, &user) {
                    Ok(()) => println!("Job is done..."),
                    Err(e) => println!("Incorrect: {:?}", e)
                };
            },
            2 => {
                let out_user: &User;
                let mut login = String::new();
                let mut password = String::new();

                println!("Введите логин: ");
                let _ = io::stdin().read_line(&mut login);
                println!();

                println!("Введите пароль: ");
                let _ = io::stdin().read_line(&mut password);
                println!();

                match actions_with_db::db::Select(&mut client) {
                    Ok(vec) => {
                        match findUser(&vec, login) {
                            Ok(out_user) => {
                                match verify_password(&out_user.password, password) {
                                    Ok(res) => {
                                        if res {
                                            println!("Вход выполнен");
                                        } else {
                                            println!("Вход невыполнен");
                                        }
                                    },
                                    Err(_e) => println!("Incorrect")
                                }
                            },
                            Err(e) => println!("Вход не выполнен: {:?}", e)
                        }
                    },
                    Err(e) => println!("Incorrect: {:?}", e)
                };
            },
            0 => {
                println!("Выход из программы.");
                break; // Выход из цикла
            },
            _ => println!("Неверный выбор, попробуйте снова."),
        }
    }
    
    Ok(())
}
