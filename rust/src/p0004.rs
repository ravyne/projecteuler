// Problem #2: Even fibonacci numbers
// https://projecteuler.net/problem=4
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.
// 10 9 8 7 6

// 2 3 5 7 =

//use std::cmp::max

// / |  9   8   7   6   5   4   3   2   1
// --+-----------------------------------
// 9 | 81  72  63  54  45  36  27  18   9
// 8 | --  64  56  48  40  32  24  16   8
// 7 | --  --  49  42  35  28  21  14   7
// 6 | --  --  --  36  30  24  18  12   6
// 5 | --  --  --  --  25  20  15  10   5
// 4 | --  --  --  --  --  16  12   8   4
// 3 | --  --  --  --  --  --   9   6   3
// 2 | --  --  --  --  --  --  --   4   2
// 1 | --  --  --  --  --  --  --  --   1

pub mod solutions {
    const LIMIT: i64 = 999;

    struct Iterator2D<T: Iterator, U: Iterator> {
        row_iter: T,
        col_iter: U,
    }

    impl<T: Iterator, U: Iterator> Iterator2D<T, U> {
        fn new(row_iter: T, col_iter: U) -> Iterator2D<T, U> {
            Iterator2D { row_iter, col_iter }
        }
    }

    impl<T: Iterator, U: Iterator> Iterator for Iterator2D<T, U> {
        type Item = (T::Item, U::Item);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(r) = self.row_iter.next() {
                if let Some(c) = self.col_iter.next() {
                    return Some((r, c));
                }
            }

            None
        }
    }

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

        palindrome
    }

    pub fn optimal() -> i64 {
        let mut palindrome: i64 = 0;
        //let mut found = false;

        for r in (1..=LIMIT).rev() {
            let s = LIMIT - r;
            let a = Iterator2D::new(r..=LIMIT, (std::cmp::max(r - s, 0)..=r).rev());
            let b = Iterator2D::new(r..=LIMIT, (std::cmp::max(r - s - 1, 0)..=r - 1).rev());

            for c in a.chain(b) {
                let prod = c.0 * c.1;
                palindrome = if is_palindrome(prod) { prod } else { 0 };

                //println!("<{}, {}>: product {} is palindrome? {} ", c.0, c.1, prod, prod == palindrome);

                if palindrome != 0 {
                    break;
                }
            }

            if palindrome != 0 {
                break;
            }
        }

        palindrome
    }

    pub fn short() -> i64 {
        let mut palindrome: i64 = 0;
        //let mut found = false;

        for r in (1..=LIMIT).rev() {
            let s = LIMIT - r;
            let a = Iterator2D::new(r..=LIMIT, (std::cmp::max(r - s, 0)..=r).rev());
            let b = Iterator2D::new(r..=LIMIT, (std::cmp::max(r - s - 1, 0)..=r - 1).rev());

            if let Some(c) = a.chain(b).find(|&c| is_palindrome(c.0 * c.1)) {
                palindrome = c.0 * c.1;
                break;
            }
        }

        palindrome
    }
}
