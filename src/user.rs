use std::{f32::consts::E, io::Error, ptr::null, vec};

pub struct User {
    pub login: String,
    pub password: String,
    pub email: String
}

pub fn findUser(vec: &Vec<User>, login: String) -> Result<&User, ()> {
    for item in vec {
        if login == item.login {
            return Ok(&item);
        }
    }

    Err(())
}