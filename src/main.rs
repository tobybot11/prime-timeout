extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

const BIG_PRIME: u64 = 15485867;

// Synchronous version
fn main() {
    if is_prime(BIG_PRIME) {
        println!("Prime");
    } else {
        println!("Not prime");
    }
}

// checks whether a number is prime, slowly
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 { return false }
    }
    true
}

