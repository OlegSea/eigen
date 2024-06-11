#![allow(clippy::cast_precision_loss)]

use num::Complex;
use thiserror::Error;

#[derive(Debug)]
pub struct CubicPolynomial {
    pub a: Complex<f32>,
    pub b: Complex<f32>,
    pub c: Complex<f32>,
    pub d: Complex<f32>,
}

#[derive(Error, Debug)]
pub enum MatrixReadError {
    #[error("Non-float value: {0}")]
    NonFloat(String),
    #[error("Wrong number of columns, expected: 3, found: {0}")]
    WrongColumns(usize),
    #[error("Wrong number of rows, expected: 3, found: {0}")]
    WrongRows(usize),
}

#[must_use]
pub const fn cf_from_int(num: i32) -> Complex<f32> {
    Complex::new(num as f32, 0.0)
}

pub type Matrix = [[f32; 3]; 3];
pub type VecMatrix = Vec<Vec<f32>>;
