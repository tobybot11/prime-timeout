extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

// use std::time::Duration;

// use futures::Future;
// use futures_cpupool::CpuPool;
// use tokio_timer::Timer;

const BIG_PRIME: u64 = 15485867;
// const BIG_PRIME: u64 = 15485868;

// Synchronous version
fn main() {
    // let pool = CpuPool::new_num_cpus();
    // let timer = Timer::default();

    // a future that resolves to Err after a timeout
    // let timeout = timer.sleep(Duration::from_millis(1))
    //     .then(|_| Err(()));

    //     // a future that resolves to Ok with the primality result
    // let prime = pool.spawn_fn(|| {
    //     Ok(is_prime(BIG_PRIME))
    // });

    // a future that resolves to one of the above values -- whichever
    // completes first!
    // let winner = timeout.select(prime).map(|(win, _)| win);

    // // now block until we have a winner, then print what happened
    // match winner.wait() {
    //     Ok(true) => println!("Prime"),
    //     Ok(false) => println!("Not prime"),
    //     Err(_) => println!("Timed out"),
    // }

    // spawn our computation, getting back a *future* of the answer
    // let prime_future = pool.spawn_fn(|| {
    //   let prime = is_prime(BIG_PRIME);

    // For reasons we'll see later, we need to return a Result here
    // let res: Result<bool, ()> = Ok(prime);
    //         res
    //   });

    // println!("Created the future");

    // unwrap here since we know the result is Ok
    //     if prime_future.wait().unwrap() {
    //       println!("Prime");
    // } else {
    //   println!("Not prime");
    // }

    if is_prime(BIG_PRIME) {
        println!("Prime!");
    } else {
        println!("Not Prime");
    }
}

// checks whether a number is prime, slowly
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 { return false }
    }
    true
}

