fn max_profit_from(arr: &[i32]) -> i32{
    let mut max_profit = 0;
    let mut smallest_day = arr[0];

    for &price in arr.iter() {
        if price < smallest_day {
            smallest_day = price;
        }
        let profits_today = price - smallest_day;
        if profits_today > max_profit {
            max_profit = profits_today;
        }
    }
    max_profit
}

fn main() {
    let profit = max_profit_from(&[3,1,10,0,1]);
    println!("{}",profit);
}
