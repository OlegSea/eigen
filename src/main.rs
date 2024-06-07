#![allow(clippy::cast_precision_loss)]

use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use num::Complex;

#[derive(Debug)]
struct CubicPolynomial {
    a: Complex<f32>,
    b: Complex<f32>,
    c: Complex<f32>,
    d: Complex<f32>,
}

#[derive(Debug, Clone)]
struct MatrixReadError;

impl Display for MatrixReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid matrix in the file")
    }
}

fn cf_from_int(num: i32) -> Complex<f32> {
    Into::<Complex<f32>>::into(num as f32)
}

// #[derive(Debug)];
type Matrix = [[f32; 3]; 3];

fn read_file(file_path: &Path) -> String {
    read_to_string(file_path).unwrap()
}

fn matrix_from_string(matrix_text: &str) -> Result<Matrix, MatrixReadError> {
    if matrix_text.lines().count() != 3 {
        return Err(MatrixReadError);
    }

    let lines = matrix_text.lines().map(|line| {
        std::convert::TryInto::<[f32; 3]>::try_into(
            line.split(' ')
                .map(|x| x.parse::<f32>().unwrap())
                .collect::<Vec<f32>>(),
        )
        .map_err(|_| MatrixReadError)
    });

    if lines.clone().any(|x| x.is_err()) {
        return Err(MatrixReadError);
    }

    Ok(lines
        .flatten()
        .collect::<Vec<[f32; 3]>>()
        .try_into()
        .unwrap())
}

fn polynomial_from_matrix(m: &Matrix) -> CubicPolynomial {
    CubicPolynomial {
        d: (m[0][1] * m[1][0])
            .mul_add(
                -m[2][2],
                (m[0][0] * m[1][2]).mul_add(
                    -m[2][1],
                    (m[1][1] * m[0][2]).mul_add(
                        -m[2][0],
                        (m[0][2] * m[1][0]).mul_add(
                            m[2][1],
                            (m[1][1] * m[0][0]).mul_add(m[2][2], m[0][1] * m[1][2] * m[2][0]),
                        ),
                    ),
                ),
            )
            .into(),
        c: m[1][1]
            .mul_add(
                -m[2][2],
                m[0][0].mul_add(
                    -m[2][2],
                    m[1][1].mul_add(
                        -m[0][0],
                        m[0][1].mul_add(m[1][0], m[0][2].mul_add(m[2][0], m[1][2] * m[2][1])),
                    ),
                ),
            )
            .into(),
        b: (m[0][0] + m[1][1] + m[2][2]).into(),
        a: (-1.0).into(),
    }
}

fn find_x(
    a: Complex<f32>,
    b: Complex<f32>,
    c: Complex<f32>,
    delta_0: Complex<f32>,
    n: i32,
) -> Complex<f32> {
    let epsilon = (cf_from_int(-1) + cf_from_int(-3).sqrt()) / cf_from_int(2);
    let mc = c * epsilon.powi(n);

    match c {
        Complex::ZERO => -b / (cf_from_int(3) * a),
        _ => (-1.0 / (3.0 * a)) * (b + mc + (delta_0 / mc)),
    }
}

fn solve_cubic(p: &CubicPolynomial) -> (Complex<f32>, Complex<f32>, Complex<f32>) {
    let deltas = (
        p.b.powu(2) - cf_from_int(3) * p.a * p.c,
        cf_from_int(2) * p.b.powu(3) - cf_from_int(9) * p.a * p.b * p.c
            + cf_from_int(27) * p.a.powu(2) * p.d,
    );

    let sq: Complex<f32> = deltas.1.powu(2) - cf_from_int(4) * deltas.0.powu(3);

    let c = if deltas == (Complex::ZERO, Complex::ZERO) {
        cf_from_int(0)
    } else {
        let try_c1 = ((deltas.1 + sq.sqrt()) / 2.0).powf(1.0 / 3.0);
        if try_c1 == Complex::ZERO {
            ((deltas.1 - sq.sqrt()) / 2.0).powf(1.0 / 3.0)
        } else {
            try_c1
        }
    };

    let v: Vec<Complex<f32>> = (0..=2).map(|n| find_x(p.a, p.b, c, deltas.0, n)).collect();
    (v[0], v[1], v[2])
}

fn main() {
    println!("Hello, world!");
    let pathbuf = PathBuf::from("input.txt");
    let file_text = read_file(&pathbuf);
    let matrix = matrix_from_string(&file_text).unwrap();

    println!("{matrix:?}");

    let polynomial = polynomial_from_matrix(&matrix);

    println!("{polynomial:?}");

    let r = solve_cubic(&polynomial);

    println!("{r:?}");
}
