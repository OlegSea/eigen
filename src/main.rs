use std::path::PathBuf;

use eigen::{
    polynomial_from_matrix,
    solve_cubic,
    io::file_into_matrix
};

fn main() {
    let pathbuf = PathBuf::from("input.txt");

    let matrix = file_into_matrix(&pathbuf).unwrap();

    println!("{matrix:?}");

    let polynomial = polynomial_from_matrix(&matrix);

    println!("{polynomial:?}");

    let r = solve_cubic(&polynomial);

    println!("{r:?}");
}
