pub mod db {
    use postgres::{Client, Error, NoTls};
    use postgres::types::Type;
    use crate::user::{self, User};
    

    pub fn Insert(client: &mut Client, user: &user::User) -> Result<(), Error> {
        let statement = client.prepare_typed(
            "INSERT INTO users (login,  password, email) VALUES ($1, $2, $3)",
            &[Type::VARCHAR, Type::TEXT, Type::VARCHAR]
        )?;

        let res = client.execute(
            &statement,
            &[&user.login, &user.password, &user.email]
        )?;

        Ok(())
    }

    pub fn Select(client: &mut Client) -> Result<Vec<User>, Error> {
        let mut vec = Vec::new();

        for row in client.query("SELECT * FROM users", &[])? {
            let user = User {
                login: row.get(1),
                password: row.get(2),
                email: row.get(3)
            };
            vec.push(user);
        };

        Ok(vec)
    }
}