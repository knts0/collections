fn main() {
    println!("Hello, world!");

    // vector
    let mut v1 = vec![10, 4, 8, 9, 32];
    let mut v2 = vec![10, 4, 8, 9, 32, 5];
    let result1 = calc_median(&mut v1);
    let result2 = calc_median(&mut v2);
    println!("{}", *result1);
    println!("{}", *result2);
}

fn calc_median(vector: &mut Vec<i32>) -> &i32 {
    vector.sort();
    let length = vector.len();
    &vector[length / 2]
}
