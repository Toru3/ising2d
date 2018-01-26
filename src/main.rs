extern crate rand;
extern crate num;

use std::vec::Vec;
use rand::Rng;
use rand::SeedableRng;
use rand::distributions::{IndependentSample, Range};
use std::time::Instant;
use std::fs;
use std::io::{BufWriter, Write};

fn rand_ising2d(n: usize, rng: &mut rand::XorShiftRng) -> Vec<i32> {
    let mut a = Vec::with_capacity(n * n);
    let between = Range::new(0, 2);
    for _ in 0..n * n {
        a.push(between.ind_sample(rng) * 2 - 1);
    }
    a
}

fn ising2d_sum_of_adjacent_spins(s: &Vec<i32>, m: usize, n: usize, i: usize, j: usize) -> i32 {
    s[if i < m - 1 { i + 1 } else { 0 } * n + j] + s[if i > 0 { i - 1 } else { m - 1 } * n + j] +
        s[i * n + if j < n - 1 { j + 1 } else { 0 }] +
        s[i * n + if j > 0 { j - 1 } else { n - 1 }]
}

fn ising2d_sweep(
    m: usize,
    n: usize,
    s: &mut Vec<i32>,
    beta: f64,
    ntiers: usize,
    rng: &mut rand::XorShiftRng,
) {
    let prob: Vec<f64> = (-4..5)
        .map(|k: i32| -> f64 { (-2. * beta * (k as f64)).exp() })
        .collect();
    for _ in 0..(ntiers / (m * n)) {
        for i in 0..m {
            for j in 0..n {
                let r = rng.next_f64();
                let s1 = s[i * n + j];
                let k = s1 * ising2d_sum_of_adjacent_spins(s, n, m, i, j);
                s[i * n + j] = if r < prob[(k + 4) as usize] { -s1 } else { s1 };
            }
        }
    }
}

fn print_ppm(filename: &str, m: usize, n: usize, s: &Vec<i32>) {
    let mut f = BufWriter::new(fs::File::create(filename).unwrap());
    f.write(b"P1\n").unwrap();
    f.write(format!("{} {}\n", n, m).as_bytes()).unwrap();
    for i in 0..m {
        for j in 0..n {
            f.write(
                format!("{} ", if s[i * n + j] == 1 { 1 } else { 0 }).as_bytes(),
            ).unwrap();
        }
        f.write(b"\n").unwrap();
    }
    f.write(b"\n").unwrap();
}

fn main() {
    let mut t_rng = rand::thread_rng();
    let mut seed: [u32; 4] = [0; 4];
    for i in 0..4 {
        seed[i] = t_rng.next_u32();
    }
    //https://ja.wikipedia.org/wiki/Xorshift の xor128 が用いられている
    let mut rng = rand::XorShiftRng::from_seed(seed);
    let beta_crit = (1. + (2.0f64).sqrt()).ln() / 2.;
    let mut s = rand_ising2d(100, &mut rng);
    print_ppm("start.pbm", 100, 100, &s);
    let start = Instant::now();
    ising2d_sweep(100, 100, &mut s, beta_crit, 1_000_000_000, &mut rng);
    let end = Instant::now();
    let time = end.duration_since(start);
    print_ppm("result.pbm", 100, 100, &s);
    println!(
        "{}s",
        time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
    );
}
