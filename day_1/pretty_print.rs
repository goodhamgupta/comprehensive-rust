fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            if transpose[i][j] == 0 {
                transpose[i][j] = matrix[j][i];
            }
        }
    }
    return transpose;
}

// fn transpose_new<'a>(matrix: &'a [&'a [i32]]) -> &'a [&'a [i32]] {
//     // This function will not work as dynamically sized arrays are not allowed in rust
//     let mut arr_size = matrix.len();
//     let mut transpose: [[i32;arr_size];arr_size] = [[0;arr_size];arr_size];
//     for i in 0..arr_size {
//         for j in 0..arr_size {
//             if transpose[i][j] == 0 {
//                 transpose[i][j] = matrix[j][i];
//             }
//         }
//     }
//     return transpose;
// }

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("matrix:");
    pretty_print(&matrix);

    let transpose = transpose(matrix);

    println!("Matrix Transpose:");
    pretty_print(&transpose);
}
