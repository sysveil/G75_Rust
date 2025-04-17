use std::process::exit;

fn fail(){
    println!("Unbalanced!");
    exit(0);
}

fn main(){
    let seq = String::from("[()]{}");
    let mut stack = String::new();

    for ch in seq.chars() {
        if "{[(".contains(ch){
            stack.push(ch);
        } else if "}])".contains(ch){
            let last = stack.pop().unwrap_or_else(
                || {fail(); ' '}
            );
            match (last,ch){
                ('(',')') | ('[',']') | ('{','}') => {}
                _ => {fail();}
            }
        }
    }
    if !stack.is_empty(){fail();}
    println!("Balanced!");
}
