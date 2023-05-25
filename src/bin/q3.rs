// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let target : usize = 600851475143;
    let up_to = (target as f32).sqrt().ceil() as usize;
    let primes = generate_primes(up_to);

    let mut p_factors = vec!();
    for i in primes {
        if target % i == 0 {
            p_factors.push(i);
        }
    }

    println!("{:?}", p_factors);
        println!("The largest prime factor of {0} is {1}", target, *p_factors.iter().max().unwrap())

}

fn generate_primes(n: usize) -> Vec<usize> {
    let mut sieve_array = vec![true; n+1]; // Creates a vector that is true n+1 times
    sieve_array[0] = false;
    sieve_array[1] = false;

    let mut prime_array :Vec<usize> = vec!();
    for i in 2..n+1 {
        if sieve_array[i] {
            for j in (2*i..n+1).step_by(i) {
                sieve_array[j] = false;
            }
            prime_array.push(i as usize)
        }
    }
    prime_array

}

