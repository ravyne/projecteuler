// Problem #1: Multiples of 3 or 5
// https://projecteuler.net/problem=1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

pub mod solutions {

    const LIMIT: i64 = 1000;

    pub fn simple() -> i64 {
        let mut sum: i64 = 0;

        for n in 0..LIMIT {
            if n % 5 == 0 || n % 3 == 0 {
                sum += n;
            }
        }

        return sum;
    }
}
