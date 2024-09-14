fn main() {
    let x = 5;
    let x = x + 5;
    {
        let x = x * 2;
        println!("Scope of x {x}");
    }
    let number: u8 = 250;
    println!("number {number}");

    println!("Outside scope {x}");
    let y = addition(5, 10);
    println!("{y}");

    let s = String::from("hello");
    takes_ownership(s);
    let t = 5;
    makes_copy(t);

    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 {s1}");
    //println!("s2 {s2}");
    println!("s3 {s3}");
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}

fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}

fn division(x: i32, y: i32) -> i32 {
    x / y
}

fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
