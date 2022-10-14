// Problem #2: Even fibonacci numbers
// https://projecteuler.net/problem=4
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

//use std::cmp::max;

pub mod solutions {
    const LIMIT: i64 = 999;

    fn is_palindrome(number: i64) -> bool {
        let fwd: String = number.to_string();
        let rev: String = fwd.chars().rev().collect();

        fwd.eq(&rev)
    }

    pub fn simple() -> i64 {
        let mut palindrome: i64 = 0;

        for a in (1..=LIMIT).rev() {
            for b in (a..=LIMIT).rev() {
                let prod = a * b;

                // println!("<{}, {}> = {}", a, b, prod);

                if is_palindrome(prod) {
                    palindrome = std::cmp::max(prod, palindrome);
                }
            }
        }

        return palindrome;
    }
}
