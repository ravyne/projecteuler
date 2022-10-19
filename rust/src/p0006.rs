// Problem #6: Sum square difference
// https://projecteuler.net/problem=6
//
// The sum of the squares of the first ten natural numbers is:
//
//   (1^2 + 2^2 + ... + 10^2) = 385
//
// The square of the sum of the first ten natural numbers is:
//
//   (1 + 2 + ... + 10)^2 = 55^2 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is: 3025 - 385 = 2640
//
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

pub mod solutions {

    const LIMIT: i64 = 100;

    pub fn simple() -> i64 {
        let numbers = 1i64..=LIMIT;
        let sum_of_squares: i64 = i64::pow(numbers.clone().sum(), 2);
        let square_of_sums: i64 = numbers.clone().map(|n| n * n).sum();

        sum_of_squares - square_of_sums
    }
}
