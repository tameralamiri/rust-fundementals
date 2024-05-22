struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

// Associated functions are functions that are associated with the struct, but do not have access to the struct instance itself.
impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: String) -> Self {
        Self {
            username: email.split('@').next().unwrap().to_string(),
            email,
            uri: String::from(""),
            active: true,
        }
    }

    fn update_uri(&mut self, uri: String) {
        self.uri = uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut user_from_email = User::from_email(String::from("user@email.com"));

    user_from_email.update_uri(String::from("https://user.com"));
    println!("Hello, {}!", user_from_email.username);
}
