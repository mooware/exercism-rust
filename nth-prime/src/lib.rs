pub fn nth(n: u32) -> u32 {
    // sieve of eratosthenes as described by wikipedia
    // for i = 2, 3, 4, ..., not exceeding √n:
    //   if A[i] is true:
    //     for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n:
    //       A[j] := false.
    if n == 0 {
        return 2;
    }
    let mut sieve = Vec::new();
    sieve.resize(64, true);
    loop {
        let mut primes = 0u32;
        let len = sieve.len();
        let limit = (len as f64).sqrt() as usize;
        for i in 2..(limit+1) {
            if sieve[i] {
                primes += 1;
                if primes == n+1 {
                    return i as u32;
                }
                let mut j = i*2;
                while j < len {
                    if sieve[j] {
                        sieve[j] = false;
                    }
                    j += i;
                }
            }
        }
        // not found, resize and try again
        sieve.resize(sieve.len() * 2, true);
        dbg!(sieve.len());
    }
}
