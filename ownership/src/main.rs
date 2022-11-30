fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("s1 = {s1}");

    let s2 = s1.clone();
    println!("s2 = {s2}");
    println!("s1 = {s1}");

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);


    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    
    
    let s3 = given_ownership();
    println!("{}", s3);
}


fn given_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
