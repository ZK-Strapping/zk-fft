#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

const TRUNC_PRECISION: i32 = 10;

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let n: usize = env::read();
    let ax: Vec<f64> = env::read();
    let m: usize = env::read();
    let bx: Vec<f64> = env::read();

    // TODO: do something with the input
    let cx = poly_mul(n, ax, m, bx);

    // write public output to the journal
    env::commit(&cx);
}

use std::ops::*;

fn fft(coeff: &mut [Complex], invert: bool) {
    let n = coeff.len();

    let shift = n.leading_zeros() + 1;
    for i in 0..n {
        let j = i.reverse_bits() >> shift;
        if i < j {
            coeff.swap(i, j);
        }
    }

    let mut len = 2;

    while len <= n {
        let mut ang = std::f64::consts::TAU / len as f64;
        if invert {
            ang = -ang;
        }
        let w = Complex(ang.cos(), ang.sin());
        for i in (0..n).step_by(len) {
            let mut wi = Complex(1., 0.);
            for j in 0..len / 2 {
                let even = coeff[i + j];
                let odd = coeff[i + j + len / 2] * wi;
                coeff[i + j] = even + odd;
                coeff[i + j + len / 2] = even - odd;
                wi = wi * w;
            }
        }
        len <<= 1;
    }
    if invert {
        for coef in coeff {
            coef.0 /= n as f64;
            coef.1 /= n as f64;
        }
    }
}

fn poly_mul(n: usize, x: Vec<f64>, m: usize, y: Vec<f64>) -> Vec<f64> {
    let mut x: Vec<Complex> = x.iter().map(|xi| Complex(*xi as f64, 0.)).collect();
    let mut y: Vec<Complex> = y.iter().map(|yi| Complex(*yi as f64, 0.)).collect();

    let len = (n + m).next_power_of_two();
    x.resize(len, Complex(0., 0.));
    y.resize(len, Complex(0., 0.));

    fft(&mut x, false);
    fft(&mut y, false);

    x.iter_mut().zip(&y).for_each(|(xi, &yi)| *xi = *xi * yi);
    fft(&mut x, true);

    println!("Results of FFT & iFFT:");
    for xi in &x {
        println!("{} {}", xi.0, xi.1);
    }
    x.iter().map(|xi| corr(xi.0) as f64).collect()
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

fn corr(val: f64) -> f64 {
    let mut val2 = (val*10.0f64.powi(TRUNC_PRECISION)).round();
    if val2.abs() <= f64::EPSILON {
        val2 = 0.0;
    }
    val2 * 10.0f64.powi(-TRUNC_PRECISION)
}