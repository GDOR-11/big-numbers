use rug::Integer;
use primal::Sieve;

// translation and optimization of https://www.luschny.de/math/factorial/csharp/FactorialPrimeSwing.cs.html

pub fn factorial(n: usize) -> Integer {
    let primes = &Sieve::new(n).primes_from(3).collect();

    let exp_2 = n - n.count_ones() as usize;
    rec_factorial(primes, n) << exp_2
}


fn rec_factorial(primes: &Vec<usize>, n: usize) -> Integer {
    if n < 2 {
        Integer::from(1)
    } else {
        rec_factorial(primes, n / 2).square() * swing(primes, n)
    }
}

const SMALL_ODD_SWING: &[usize] = &[1, 1, 1, 3, 3, 15, 5, 35, 35, 315, 63, 693, 231, 3003, 429, 6435, 6435, 109395,
                                    12155, 230945, 46189, 969969, 88179, 2028117, 676039, 16900975, 1300075, 35102025,
                                    5014575, 145422675, 9694845, 300540195, 300540195];

fn swing(primes: &Vec<usize>, n: usize) -> Integer {
    if n < SMALL_ODD_SWING.len() {
        return Integer::from(SMALL_ODD_SWING[n]);
    }

    let mut result = Integer::from(1);

    let root_n = (n as f64).sqrt().floor() as usize;
    let n_over_3 = n / 3;

    for &prime in primes {
        if prime > root_n {
            if prime > n_over_3 { break; }
            if (n / prime) % 2 == 1 {
                result *= prime;
            }
            continue;
        }

        let mut q = n;
        while q > 0 {
            q /= prime;
            if q % 2 == 1 {
                result *= prime;
            }
        }
    }

    let primorial = primorial(primes, n / 2 + 1, n);

    result * primorial
}

// doing this is faster than directly calling .iter().product()
fn primorial(primes: &Vec<usize>, start: usize, end: usize) -> Integer {
    if end - start < 2048 {
        // if the length is small enough, multiply everything directly (more cache efficiency,
        // less recursion overhead, etc.)
        let start_index = match primes.binary_search(&start) { Ok(i) => i, Err(i) => i };
        let end_index = match primes.binary_search(&end) { Ok(i) => i + 1, Err(i) => i };

        primes[start_index..end_index].iter().product()
    } else {
        // otherwise, multiply each half separately and then multiply the results together.
        // this is better than a simple for loop because it only makes one big multiplication,
        // while a for loop would do one big multiplication for each element close to the end
        primorial(primes, start, (start + end) / 2) * primorial(primes, 1 + (start + end) / 2, end)
    }
}
