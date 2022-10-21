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
//   (a + b)*(a + b) = aa + bb + 2ab
//   (a + b + c)*(a + b +  c) = aa + ab + ac + ba + bb + bc + ca + cb + cc
//                            = aa + bb + cc + 2ab + 2ac + 2bc
//   (a+b+c+d)*(a+b+c+d) = aa + ab + ac + ad + ab + bb + bc + bd + ac + bc + cc + cd + ad + bd + cd + dd
//                       = aa + bb + cc + dd + 2ab + 2ac + 2ad + 2bc + 2bd + 2cd

// Hence the difference between the sum of the squares of the first ten natural
// numbers and the square of the sum is: 3025 - 385 = 2640
//
// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

pub mod solutions {

    const LIMIT: i64 = 100;

    pub fn simple() -> i64 {
        let numbers = 1i64..=LIMIT;
        let sum_of_squares: i64 = numbers.clone().map(|n| n * n).sum();
        let square_of_sums: i64 = i64::pow(numbers.clone().sum(), 2);

        square_of_sums - sum_of_squares
    }

    pub fn optimal() -> i64 {
        let n: i64 = LIMIT;
        let sum_n = n * (n+1) / 2;
        let square_of_sums = sum_n * sum_n;
        let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;

        square_of_sums - sum_of_squares
    }

    // term_elim/increm and short/increm are derived by expanding the square of
    // sums, and canceling common terms from the sum of squares; this leaves only
    // the non-square terms (2ab & friends), but computing them alone seems to be
    // about the same amount of work as computing the sum of squares and square
    // of sums directly.
    pub fn term_elim() -> i64 {
        let mut diff: i64 = 0;
        let all_sum: i64 = (1i64..=LIMIT).sum();

        for i in 1i64..=LIMIT {
            // all_sum - i is equivalent to (1..=LIMIT).filter(|&S| s!=i).sum()
            //  diff += i * (1i64..=LIMIT).filter(|&s| s != i).sum::<i64>();
            diff += i * (all_sum - i);
        }

        diff
    }

    pub fn short() -> i64 {
        let all_sum: i64 = (1i64..=LIMIT).sum();

        (1i64..=LIMIT).fold(0, |acc, i| acc + i * (all_sum - i))
    }

    pub fn term_elim_increm() -> i64 {
        let mut diff: i64 = 0;
        let mut all_sum: i64 = (1i64..=LIMIT).sum();

        for i in 1i64..=LIMIT {
            // all_sum -= i is equivalent to (i+1..==LIMIT).sum()
            //  diff += i * (i+1..=LIMIT).sum::<i64>();
            all_sum -= i;
            diff += i * all_sum
        }

        2 * diff
    }

    pub fn short_increm() -> i64 {
        let mut all_sum: i64 = (1i64..=LIMIT).sum();

        2 * (1i64..=LIMIT).fold(0, |acc, i| { all_sum -= i; acc + i * all_sum })
    }
}
