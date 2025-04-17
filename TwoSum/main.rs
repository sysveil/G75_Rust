use std::{collections::HashMap, process::exit};

fn main() {
    let source_arr: [i32; 4] = [2, 7, 11, 15];
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    let target: i32 = 9;

    for (idx,val) in source_arr.iter().enumerate(){

        let diff: i32 = target - val;
        if let Some(res) = hashmap.get(&diff){
            println!("Target found! The two indices are {},{}", &idx, &res);
            exit(0);
        }
        hashmap.insert(*val, idx);
    }
}
