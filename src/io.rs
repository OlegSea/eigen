use std::{fs::read_to_string, path::Path};

use crate::types::{cf_from_int, Matrix, MatrixReadError, VecMatrix};
use itertools::Itertools;
use num::Complex;

/// .
///
/// # Errors
///
/// This function will return an error if the numbers in a matrix aren't really numbers.
pub fn vec_matrix_from_string(matrix_text: &str) -> Result<VecMatrix, MatrixReadError> {
    let vec_matrix = matrix_text
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|x| {
                    x.parse::<Complex<f32>>()
                        .map_err(|_| MatrixReadError::NonFloat(x.to_string()))
                })
                .try_collect()
        })
        .try_collect()?;
    Ok(vec_matrix)
}

/// .
///
/// # Errors
///
/// This function will return an error if there is a wrong number of rows/columns in a given `VecMatrix`.
pub fn matrix_from_vec(matrix_vec: &VecMatrix) -> Result<Matrix, MatrixReadError> {
    let rows = matrix_vec.len();
    if rows != 3 {
        return Err(MatrixReadError::WrongRows(rows));
    }

    let mut matrix: Matrix = [[cf_from_int(0); 3]; 3];

    for (i, row) in matrix_vec.iter().enumerate() {
        let cols = row.len();
        if cols != 3 {
            return Err(MatrixReadError::WrongColumns(cols));
        }

        for (j, col) in row.iter().enumerate() {
            matrix[i][j] = *col;
        }
    }

    Ok(matrix)
}

/// .
///
/// # Errors
///
/// This function will return an error if:
/// - The file could not be opened.
/// - The file contains characters that could not be parsed into f32.
/// - The number of rows/columns in a file is wrong.
/// - idk stars misaligned (skill isseu).
pub fn file_into_matrix(path: &Path) -> anyhow::Result<Matrix> {
    let file_text = read_to_string(path)?;

    let vec_matrix = vec_matrix_from_string(&file_text)?;

    let matrix = matrix_from_vec(&vec_matrix)?;
    Ok(matrix)
}
