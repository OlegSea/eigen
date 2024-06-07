use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct CubicPolynomial {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Debug, Clone)]
struct MatrixReadError;

impl Display for MatrixReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid matrix in the file")
    }
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
        d: m[1][1] * m[0][0] * m[2][2] + m[0][1] * m[1][2] * m[2][0] + m[0][2] * m[1][0] * m[2][1]
            - m[1][1] * m[0][2] * m[2][0]
            - m[0][0] * m[1][2] * m[2][1]
            - m[0][1] * m[1][0] * m[2][2],
        c: m[0][2] * m[2][0] + m[1][2] * m[2][1] + m[0][1] * m[1][0]
            - m[1][1] * m[0][0]
            - m[0][0] * m[2][2]
            - m[1][1] * m[2][2],
        b: m[0][0] + m[1][1] + m[2][2],
        a: -1.0,
    }
}

fn main() {
    println!("Hello, world!");
    let pathbuf = PathBuf::from("input.txt");
    let file_text = read_file(&pathbuf);
    let matrix = matrix_from_string(&file_text).unwrap();

    println!("{matrix:?}");

    let polynomial = polynomial_from_matrix(&matrix);

    println!("{polynomial:?}");
}
