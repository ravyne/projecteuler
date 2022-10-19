mod p0001;
mod p0002;
mod p0004;
mod p0005;
mod p0006;

fn main() {
    println!("Starting Project Euler...");

    println!("  Problem #1: Multiples of 3s or 5s...");
    println!("    Solution #1.1: simple -- {}", p0001::solutions::simple());
    println!("    Solution #1.2: short -- {}", p0001::solutions::short());
    println!();

    println!("  Problem #2: Even Fibonacci numbers...");
    println!("    Solution #2.1: simple -- {}", p0002::solutions::simple());
    println!("    Solution #2.2: take_while -- {}", p0002::solutions::using_take_while());
    println!("    Solution #2.3: iterators -- {}", p0002::solutions::using_iterators());
    println!();

    println!("  Problem #4: Largest palindrome product...");
    println!("    Solution #4.1: simple -- {}", p0004::solutions::simple());
    println!("    Solution #4.3: optimal == {}", p0004::solutions::optimal());
    println!("    Solution #4.3: short == {}", p0004::solutions::short());
    println!();

    println!("  Problem #5: Smallest multiple...");
    println!("    Solution #5.1: simple -- {}", p0005::solutions::simple());
    println!();

    println!("  Problem #6: Sum square difference...");
    println!("    Solution #6.1: simple -- {}", p0006::solutions::simple());
    println!();

    println!("----------");
    println!("Complete.");
}
