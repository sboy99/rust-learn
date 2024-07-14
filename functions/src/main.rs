fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let a = 10;
    let b = 20;
    let sum = do_sum(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
}
