// Problem #5: Smallest multiple
// https://projecteuler.net/problem=5
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//
// 10 9 8 7 6 5 4 3 2 1
// 10 9 8 7 6 - - - - -
// 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1
// 20 19 18 17 16 15 14 13 12 11 -- - - - - - - - - -

pub mod solutions {

    const LIMIT: i64 = 20;

    // least common multiple
    pub fn lcm(a: i64, b: i64) -> i64 {
        let (l, g) = if a < b { (a, b) } else { (b, a) };

        (g..).step_by(g as usize).find(|&m| (m % l) == 0).unwrap()
    }

    pub fn simple() -> i64 {
        let numbers: [i64; LIMIT as usize] = core::array::from_fn(|n| 1 + n as i64);
        //let numbers = (1..=LIMIT).collect::<Vec<i64>>();

        numbers.into_iter().reduce(lcm).unwrap()
    }
}
