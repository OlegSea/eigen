use std::path::PathBuf;

use eigen::{find_eigenvec, io::file_into_matrix, polynomial_from_matrix, solve_cubic};
use itertools::Itertools;

fn main() {
    let pathbuf = PathBuf::from("input.txt");

    let matrix = file_into_matrix(&pathbuf).unwrap();

    println!("{matrix:?}");

    let polynomial = polynomial_from_matrix(&matrix);

    println!("{polynomial:?}");

    let eigenvalues = solve_cubic(&polynomial);

    println!("{eigenvalues:?}");

    for i in eigenvalues {
        let vecs = find_eigenvec(&matrix, i);
        println!("Eigenvectors for value {i:?}: {vecs:?}");
    }
}
