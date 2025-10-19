
fn main() {
    
    let mut matrix = vec![vec![1, 1, 1], 
    vec![1,0, 1], 
    vec![1,1,1]];

    set_matrix_zero(&mut matrix);

    println!("Result:");
    // &matrix creates an iterator over references (&Vec<i32>)
    for row in &matrix {
        println!("{:?}", row);
    }
    
}

fn set_matrix_zero(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
   
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new
    // https://users.rust-lang.org/t/i32-vs-isize-u32-vs-usize/22657/7
    let mut zero_rows: Vec<usize> = Vec::new();
    let mut zero_cols: Vec<usize> = Vec::new();

    // Find zero positions

    // for i in  0..matrix.len() {
    //     for j in 0..matrix[0].len() {

    //         if matrix[i][j] == 0 {
    //             zero_rows.push(i)
    //             zero_cols.push(j)
    //         }

    //     }
    // }
    for i in  0..rows {
        for j in 0..cols {

            if matrix[i][j] == 0 {
                zero_rows.push(i);
                zero_cols.push(j);
            }

        }
    }
    let zero_rows_len = zero_rows.len();
    let zero_cols_len = zero_cols.len();
    // for &r in &zero_rows {
    //     for j in 0..cols {
    //         matrix[r][j] = 0;
    //     }
    // }
    
    // for &c in &zero_cols {
    //     for j in 0..rows {
    //         matrix[j][c] = 0;
    //     }
    // }
    
    for i in 0..zero_rows_len {
        for j in 0..cols {
            let r = zero_rows[i];
            matrix[r][j] = 0;
        }
    }
    for i in 0..zero_cols_len {
        for j in 0..rows {
            let c = zero_cols[i];
            matrix[j][c] = 0;
        }
    }
}