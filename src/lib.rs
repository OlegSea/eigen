use itertools::Itertools;
use num::traits::MulAdd;
use num::Complex;
use types::{cf_from_int, CubicPolynomial, EigenVec, Matrix};

pub mod io;
pub mod types;

#[must_use]
pub fn polynomial_from_matrix(m: &Matrix) -> CubicPolynomial {
    CubicPolynomial {
        d: (m[0][1] * m[1][0]).mul_add(
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
        ),
        c: m[1][1].mul_add(
            -m[2][2],
            m[0][0].mul_add(
                -m[2][2],
                m[1][1].mul_add(
                    -m[0][0],
                    m[0][1].mul_add(m[1][0], m[0][2].mul_add(m[2][0], m[1][2] * m[2][1])),
                ),
            ),
        ),
        b: (m[0][0] + m[1][1] + m[2][2]),
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

#[must_use]
pub fn solve_cubic(p: &CubicPolynomial) -> Vec<Complex<f32>> {
    let deltas = (
        p.b.powu(2) - cf_from_int(3) * p.a * p.c,
        cf_from_int(2) * p.b.powu(3) - cf_from_int(9) * p.a * p.b * p.c
            + cf_from_int(27) * p.a.powu(2) * p.d,
    );

    let square_root: Complex<f32> = deltas.1.powu(2) - cf_from_int(4) * deltas.0.powu(3);

    let c = if deltas == (Complex::ZERO, Complex::ZERO) {
        cf_from_int(0)
    } else {
        let try_c1 = ((deltas.1 + square_root.sqrt()) / 2.0).powf(1.0 / 3.0);
        if try_c1 == Complex::ZERO {
            ((deltas.1 - square_root.sqrt()) / 2.0).powf(1.0 / 3.0)
        } else {
            try_c1
        }
    };

    let mut result = Vec::<Complex<f32>>::new();

    (0..=2)
        .map(|n| find_x(p.a, p.b, c, deltas.0, n))
        .for_each(|x| {
            if !result.contains(&x) {
                result.push(x);
            }
        });
    result
}

#[must_use]
pub fn singular_matrix_from_matrix(m: &Matrix, eigenvalue: Complex<f32>) -> Matrix {
    let mut result_matrix = *m;

    for (i, _) in m.iter().enumerate() {
        result_matrix[i][i] -= eigenvalue;
    }

    result_matrix
}


// TODO: блять
#[must_use]
pub fn find_eigenvec(m: &Matrix, eigenvalue: Complex<f32>) -> EigenVec {
    let sm = singular_matrix_from_matrix(m, eigenvalue);

    let p =
        (sm[1][2] * sm[0][0] - sm[0][2] * sm[1][0]) / (sm[1][1] * sm[0][0] - sm[0][1] * sm[1][0]);

    [
        -sm[0][2] / sm[0][0] + (sm[0][1] / sm[0][0]) * p,
        -p,
        cf_from_int(1),
    ]
}
