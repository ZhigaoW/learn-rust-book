#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print_email(&self) {
        println!("{}", self.email);
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("zhigaowang@zju.edu.cn"),
        username: String::from("Zhigao Wang"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("math936@163.com");

    let user2 = build_user(String::from("Aimiskamei@gmail.com"), String::from("Aimiskamei"));

    println!("{:#?}", user1);
    println!("{:#?}", user2);

    user1.print_email();

}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 2,
    }
}














