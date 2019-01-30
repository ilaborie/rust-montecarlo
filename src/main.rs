use rand::distributions::{Distribution, Uniform};
use stopwatch::Stopwatch;

use rust_montecarlo::*;

fn main() {
    let count = 10_000_000;

    let range = Uniform::new(-1.0f64, 1.0);
    let mut rng = rand::thread_rng();

    let supplier = || {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);
        Point::new(x, y)
    };

    let mut sw = Stopwatch::start_new();
    let (pi1, s) = loop_for(count, supplier);
    let d1 = sw.elapsed_ms();

    sw.restart();
    let (pi2, _s2) = loop_iter(count, s);
    let d2 = sw.elapsed_ms();

    println!("for loop: {} tooks {}ms", pi1, d1);
    println!("iter loop: {} tooks {}ms", pi2, d2);
}
