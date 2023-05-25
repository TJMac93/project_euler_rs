// If we list all the natural numbers below 10 that are multiples of 3
// or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000

fn main() {
    let mut cap = 999;
    let mut total = 0;
    while cap >0 {
        if cap % 3 == 0 || cap % 5 == 0 {
        total += cap;
        }
        cap -= 1;
    }
    println!( "{}", total)
}

