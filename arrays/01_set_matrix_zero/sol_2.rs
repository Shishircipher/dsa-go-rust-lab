use std::collections::HashSet;

fn main() {
    let mut matrix = vec![
        vec![1, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];

    set_matrix_zero(&mut matrix);

    println!("Result:");
    for row in &matrix {
        println!("{:?}", row);
    }
}

fn set_matrix_zero(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    // https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.insert
    let mut zero_rows = HashSet::new();
    let mut zero_cols = HashSet::new();

    // Step 1: Find zero positions
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                zero_rows.insert(i);
                zero_cols.insert(j);
            }
        }
    }

    // Step 2: Zero out rows
    for &r in &zero_rows {
        for j in 0..cols {
            matrix[r][j] = 0;
        }
    }

    // Step 3: Zero out columns
    for &c in &zero_cols {
        for i in 0..rows {
            matrix[i][c] = 0;
        }
    }
}
