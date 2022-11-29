
const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    println!("THREE_HOURS_IN_SECOND = {THREE_HOURS_IN_SECOND}");

    let x = x + 1;
    println!("x = {x}");
    {
        let x = x + 2;
        println!("x = {x}");
    }
    println!("x = {x}");

    let spaces = "    ";
    println!("spaces = {spaces}");
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    for i in 0..5 {
        println!("index in {i}");
    }

    let x = temconvert(x);
    println!("x = {x}");

    let feb = febonacci(20);
    println!("feb = {feb}");
}

fn temconvert(tmp: i32) -> i32 {
    return tmp + 3;
}

fn febonacci(n: i32) -> i32 {
    if n < 0 {
        return -1;
    } else if n == 0 || n == 1 {
        return 1
    } 
    febonacci(n - 1) + febonacci(n -2)
}








