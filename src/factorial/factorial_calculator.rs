//! this crate is a translation and optimization of https://www.luschny.de/math/factorial/csharp/FactorialPrimeSwing.cs.html,
//! and its only public function is factorial(n: usize) -> rug::Integer, which calculates the factorial of n in an efficient manner

use rug::Integer;
use primal::Sieve;


/// calculates the factorial of n using the factorial prime swing algorithm
pub fn factorial(n: usize) -> Integer {
    let primes = &Sieve::new(n).primes_from(3).collect();

    let exp_2 = n - n.count_ones() as usize;
    odd_factorial(primes, n) << exp_2
}


// "rec_factorial" in the original source code
fn odd_factorial(primes: &Vec<usize>, n: usize) -> Integer {
    if n < 2 {
        Integer::from(1)
    } else {
        odd_factorial(primes, n / 2).square() * odd_swing(primes, n)
    }
}

const SMALL_ODD_SWING: &[usize] = &[1, 1, 1, 3, 3, 15, 5, 35, 35, 315, 63, 693, 231, 3003, 429, 6435, 6435, 109395,
                                    12155, 230945, 46189, 969969, 88179, 2028117, 676039, 16900975, 1300075, 35102025,
                                    5014575, 145422675, 9694845, 300540195, 300540195];

// "swing" in the original source code
fn odd_swing(primes: &Vec<usize>, n: usize) -> Integer {
    if n < SMALL_ODD_SWING.len() {
        return Integer::from(SMALL_ODD_SWING[n]);
    }

    // some magic fuckery going on down there, idk exactly why it works

    let mut prime_list = vec![];
    let mut prime_list_max = 0;

    let root_n = (n as f64).sqrt().floor() as usize;
    let n_over_3 = n / 3;

    for &prime in primes {
        if prime > root_n {
            if prime > n_over_3 { break; }
            if (n / prime) % 2 == 1 {
                prime_list.push(prime);
                if prime > prime_list_max { prime_list_max = prime; }
            }
            continue;
        }

        let (mut p, mut q) = (1, n);
        while q > 0 {
            q /= prime;
            if q % 2 == 1 {
                p *= prime;
            }
        }

        if p > 1 {
            prime_list.push(p);
            if prime > prime_list_max { prime_list_max = prime; }
        }
    }

    product(&prime_list, 0, prime_list_max) * product(primes, n / 2 + 1, n)
}

/// multiplies all the numbers in a sorted vector that are inside the range start..=end
///
/// notice that if the number is not sorted this function will output rubbish
fn product(numbers: &Vec<usize>, start: usize, end: usize) -> Integer {
    if end - start < 2048 {
        // if the interval is small enough, multiply everything directly (more cache efficiency,
        // less recursion overhead, etc.)
        let start_index = match numbers.binary_search(&start) { Ok(i) => i, Err(i) => i };
        let end_index = match numbers.binary_search(&end) { Ok(i) => i + 1, Err(i) => i };

        numbers[start_index..end_index].iter().product()
    } else {
        // otherwise, multiply each "half" separately and then multiply the results together,
        // in the spirit of doing fewer big multiplications (if we were to multiply everything
        // front-to-back, a good portion of the last few numbers would be multiplied by huge values)
        product(numbers, start, (start + end) / 2) * product(numbers, 1 + (start + end) / 2, end)
    }
}
