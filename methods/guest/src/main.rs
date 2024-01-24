#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let n: usize = env::read();
    let x: Vec<i64> = env::read();
    let y: Vec<i64> = env::read();

    // TODO: do something with the input
    let z = poly_mul(n, x, y);

    // write public output to the journal
    env::commit(&z);
}

// extern crate std;
use std::ops::*;
// use std::println;
// use std::vec::Vec;

fn fft(coefs: &mut [Complex], invert: bool) {
    let n = coefs.len();
    let shift = n.leading_zeros() + 1;
    for i in 0..n {
        let reversed = i.reverse_bits() >> shift;
        if i < reversed {
            coefs.swap(i, reversed);
        }
    }
    let mut len = 2;
    while len <= n {
        let mut angle = std::f64::consts::TAU / len as f64;
        if invert {
            angle = -angle;
        }
        let w = Complex(angle.cos(), angle.sin());
        for i in (0..n).step_by(len) {
            let mut wi = Complex(1., 0.);
            for j in 0..len / 2 {
                let even = coefs[i + j];
                let odd = coefs[i + j + len / 2] * wi;
                coefs[i + j] = even + odd;
                coefs[i + j + len / 2] = even - odd;
                wi = wi * w;
            }
        }
        len <<= 1;
    }
    if invert {
        for coef in coefs {
            coef.0 /= n as f64;
            coef.1 /= n as f64;
        }
    }
}

fn poly_mul(n: usize, x: Vec<i64>, y: Vec<i64>) -> Vec<i64> {
    let mut x: Vec<Complex> = x.iter().map(|xi| Complex(*xi as f64, 0.)).collect();
    let mut y: Vec<Complex> = y.iter().map(|yi| Complex(*yi as f64, 0.)).collect();

    let len = (n * 2).next_power_of_two();
    x.reverse();
    x.resize(len, Complex(0., 0.));
    y.extend_from_within(..);
    y.resize(len, Complex(0., 0.));
    fft(&mut x, false);
    fft(&mut y, false);
    x.iter_mut().zip(&y).for_each(|(xi, &yi)| *xi = *xi * yi);
    fft(&mut x, true);

    x.iter().map(|xi| xi.0 as i64).collect()
}

#[derive(Clone, Copy, PartialEq)]
struct Complex(f64, f64);

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}
