#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use rust_decimal::prelude::*;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let n: usize = env::read();
    let ax: Vec<f64> = env::read();
    let m: usize = env::read();
    let bx: Vec<f64> = env::read();

    // TODO: do something with the input
    let cx = poly_mul(n, ax.iter().map(|&x| Decimal::from_f64(x).unwrap()).collect(), m, bx.iter().map(|&x| Decimal::from_f64(x).unwrap()).collect());

    // write public output to the journal
    env::commit(&cx.iter().map(|&x| x.to_i64().unwrap()).collect::<Vec<i64>>());
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
        let mut ang = Decimal::TWO_PI / Decimal::from_usize(len).unwrap();
        if invert {
            ang = -ang;
        }
        let w = Complex(ang.cos(), ang.sin());
        for i in (0..n).step_by(len) {
            let mut wi = Complex(Decimal::try_from(1).unwrap(), Decimal::try_from(0).unwrap());
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
            coef.0 /= Decimal::from_usize(n).unwrap();
            coef.1 /= Decimal::from_usize(n).unwrap();
        }
    }
}

fn poly_mul(n: usize, x: Vec<Decimal>, m: usize, y: Vec<Decimal>) -> Vec<Decimal> {
    let mut x: Vec<Complex> = x.iter().map(|xi| Complex(*xi, Decimal::try_from(0).unwrap())).collect();
    let mut y: Vec<Complex> = y.iter().map(|yi| Complex(*yi, Decimal::try_from(0).unwrap())).collect();

    let len = (n + m).next_power_of_two();
    x.resize(len, Complex(Decimal::try_from(0).unwrap(), Decimal::try_from(0).unwrap()));
    y.resize(len, Complex(Decimal::try_from(0).unwrap(), Decimal::try_from(0).unwrap()));

    fft(&mut x, false);
    fft(&mut y, false);

    x.iter_mut().zip(&y).for_each(|(xi, &yi)| *xi = *xi * yi);
    fft(&mut x, true);

    for xi in &x {
        println!("{} {}", xi.0, xi.1);
    }
    x.iter().map(|&y| corr(y.0)).collect()
}

#[derive(Clone, Copy, PartialEq)]
struct Complex(Decimal, Decimal);

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

fn corr(x: Decimal) -> Decimal {
    let corr_eps: Decimal = Decimal::from(10).powi(-15);
    let corr_power: Decimal = Decimal::from(10).powi(15);
    (x * corr_power).round() * corr_eps
}