#![feature(test)]

extern crate test;

use std::collections::hash_set::HashSet;

fn calculate_n_is_prime_all_nums(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    for num in 3..max_length {
        for check in 2..num {
            if num % check == 0 {
                primes[num] = false;
                break;
            }
        }
    }
    primes
}

fn calculate_n_is_prime_stop_sqrt(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    for num in 3..max_length {
        let max_check = (num as f64).sqrt() as usize;
        for check in 2..(max_check+1) {
            if num % check == 0 {
                primes[num] = false;
                break;
            }
        }
    }
    primes
}

fn calculate_n_is_prime_sieve(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    for num in 2..max_length {
        let mut check = num;
        loop {
            check = check+num;
            if check >= max_length {
                break;
            }
            primes[check] = false;
        }
    }
    primes
}

fn calculate_n_is_prime_sieve_stop_sqrt(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    let stop = (max_length as f64).sqrt() as usize + 1;

    for num in 2..stop {
        let mut check = num;
        loop {
            check = check+num;
            if check >= max_length {
                break;
            }
            primes[check] = false;
        }
    }
    primes
}

fn calculate_n_is_prime_sieve_only_prime_check(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    let stop = (max_length as f64).sqrt() as usize + 1;

    for num in 2..stop {
        if !primes[num] {
            continue;
        }
        let mut check = num*num;
        loop {
            if check >= max_length {
                break;
            }
            primes[check] = false;
            check = check+num;
        }
    }
    primes
}

// fn calculate_n_is_prime_only_prime_check(max_length: usize) -> Vec<u32> {
//     let mut primes: Vec<u32> = vec![0; max_length];
//     let mut idx = 0;
//     primes[idx] = 2;
//     idx += 1;

//     for num in 3..max_length {
//         let mut is_prime = true;
//         let stop = ((max_length as f64).sqrt() as u32) + 1;
//         for prime in primes.iter() {
//             if *prime == 0 { 
//                 break;
//             }
//             if prime > &stop {
//                 break;
//             }
//             if num % (*prime as usize) == 0 {
//                 is_prime = false;
//             }
//         }
//         if is_prime {
//             primes[idx] = num as u32;
//             idx += 1;
//         }
//     }
//     primes
// }

fn sieve_with_iterators(max_length: usize) -> Vec<bool> {
    if max_length < 2 {
        return vec![false; max_length];
    }
    let mut primes: Vec<bool> = vec![true; max_length];
    primes[0] = false;
    primes[1] = false;
    primes[2] = true;

    let stop = (max_length as f64).sqrt() as usize;

    for num in 2..=stop {
        if !primes[num] {
            continue;
        }
        ((num*num)..max_length).step_by(num)
            .for_each(|ele| primes[ele] = false);
    }
    primes
}

fn print_vec_with_index(vec: &Vec<bool>) {
    for num in 0..vec.len() {
        println!("Num: {} {} prime", num, if vec[num] { "is" } else { "is not"});
    }
}

fn main() {
    let results = calculate_n_is_prime_all_nums(10);
    print_vec_with_index(&results);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn division() {
        let num = 15;
        assert_eq!(7, num/2);
    }

    #[test]
    fn sqrt_nine() {
        let num = 9;
        let sqrt = (num as f64).sqrt() as u32;
        assert_eq!(sqrt, 3);
    }

    #[test]
    fn sqrt_ten() {
        let num = 10;
        let sqrt = (num as f64).sqrt() as u32;
        assert_eq!(sqrt, 3);
    }

    #[test]
    fn calculate_n_is_prime_all_nums_first_ten() {
        let max_length = 10;
        let vec = calculate_n_is_prime_all_nums(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }

    #[test]
    fn calculate_n_is_prime_stop_sqrt_first_ten() {
        let max_length = 10;
        let vec = calculate_n_is_prime_stop_sqrt(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }

    #[test]
    fn calculate_n_is_prime_sieve_first_ten() {
        let max_length = 10;
        let vec = calculate_n_is_prime_sieve(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }

    #[test]
    fn calculate_n_is_prime_sieve_stop_sqrt_first_ten() {
        let max_length = 10;
        let vec = calculate_n_is_prime_sieve_stop_sqrt(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }

    #[test]
    fn calculate_n_is_prime_sieve_only_prime_check_first_ten() {
        let max_length = 10;
        let vec = calculate_n_is_prime_sieve_only_prime_check(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }

    #[test]
    fn sieve_with_iterators_first_ten() {
        let max_length = 10;
        let vec = sieve_with_iterators(max_length);

        assert_eq!(vec[0], false, "zero is not prime");
        assert_eq!(vec[1], false, "one is not prime");
        assert_eq!(vec[2], true, "two is prime");
        assert_eq!(vec[3], true, "three is prime");
        assert_eq!(vec[4], false, "four is not prime");
        assert_eq!(vec[5], true, "five is prime");
        assert_eq!(vec[6], false, "six is not prime");
        assert_eq!(vec[7], true, "seven is prime");
        assert_eq!(vec[8], false, "eight is not prime");
        assert_eq!(vec[9], false, "nine is not prime");
    }


    // #[bench]
    fn calculate_n_is_prime_all_nums_bench(b: &mut Bencher) {
        let max_length = 30000;
        b.iter(|| calculate_n_is_prime_all_nums(max_length));
    }

    // #[bench]
    fn calculate_n_is_prime_stop_sqrt_bench(b: &mut Bencher) {
        let max_length = 700000;
        b.iter(|| calculate_n_is_prime_stop_sqrt(max_length));
    }

    // #[bench]
    fn calculate_n_is_prime_sieve_bench(b: &mut Bencher) {
        let max_length = 9000000;
        b.iter(|| calculate_n_is_prime_sieve(max_length));
    }

    // #[bench]
    fn calculate_n_is_prime_sieve_stop_sqrt_bench(b: &mut Bencher) {
        let max_length = 12000000;
        b.iter(|| calculate_n_is_prime_sieve_stop_sqrt(max_length));
    }

    #[bench]
    fn calculate_n_is_prime_sieve_only_prime_check_bench(b: &mut Bencher) {
        let max_length = 22000000;
        b.iter(|| calculate_n_is_prime_sieve_only_prime_check(max_length));
    }

    // #[bench]
    fn sieve_with_iterators_bench(b: &mut Bencher) {
        let max_length = 22000000;
        b.iter(|| sieve_with_iterators(max_length));
    }
}
