fn preprocess(arg: String) -> String {
    let mut res = String::new();
    for ch in arg.chars(){
        if !ch.is_alphanumeric(){ continue; }
        res.push(ch.to_ascii_lowercase());
    }
    res
}

fn is_palindrome(arg: &str) -> bool {
    arg == arg.chars().rev().collect::<String>()
}

fn main() {
    let balls = String::from("ABCBAd");
    println!("{}", is_palindrome(&preprocess(balls)));
}
