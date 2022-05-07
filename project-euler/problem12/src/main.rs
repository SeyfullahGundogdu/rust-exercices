/**
 * The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. 
 * 
 * The first ten terms would be:1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 * Let us list the factors of the first seven triangle numbers:
 *  1: 1
 *  3: 1,3
 *  6: 1,2,3,6
 * 10: 1,2,5,10
 * 15: 1,3,5,15
 * 21: 1,3,7,21
 * 28: 1,2,4,7,14,28
 * 
 * We can see that 28 is the first triangle number to have over five divisors.
 * 
 * What is the value of the first triangle number to have over five hundred divisors?

 */
fn main() {
    println!("{}",triangle(500));
}
fn triangle(divisor_count:usize) -> u64 {
    let mut inc = 1;
    let mut num = 0;
    let mut divis_ct = divisors(num);
    while divis_ct < divisor_count {
        num += inc;
        inc += 1;
        divis_ct = divisors(num);
    }
    num
}


fn divisors(num:u64) -> usize {
    let mut div_count = 2;
    let numsq = (num as f64).sqrt().ceil() as u64;
    for i in 2..=numsq {
        if num % i == 0 {
            div_count += 2;
        }
    }
    if numsq * numsq == num {
        div_count -= 1;
    }
    div_count
}