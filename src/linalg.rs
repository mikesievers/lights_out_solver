use crate::finite_field::GFElement;
use itertools::Itertools;
use std::fmt::Display;

pub struct Matrix {
    rows: Vec<Vec<GFElement>>,
}

impl Matrix {
    pub fn new(rows: Vec<Vec<GFElement>>) -> Self {
        Matrix { rows }
    }

    pub fn to_rref(&self) -> Matrix {
        // Convert the matrix to reduced row echelon form
        let n_rows = self.rows.len();
        let n_cols = self.rows[0].len();

        let mut new_rows = self.rows.clone();

        // Generate reduced row echelon form by walking through the columns
        'next_row: for row_idx in 0..self.rows.len() {
            // Verify that the first column starts with a non-zero number
            for col_idx in 0..self.rows[0].len() {
                // First, if the first column does not start with a non-zero number,
                // try to find a row that does

                match new_rows[row_idx][col_idx].value {
                    // If zero, check if another column is not null and then swap
                    0 => {
                        for lower_row_idx in row_idx + 1..n_rows {
                            if new_rows[lower_row_idx][col_idx].value != 0 {
                                // A lower row has a non-zero element in the leading column,
                                // swap, normalize and zero the rows below it
                                new_rows.swap(row_idx, lower_row_idx);
                            }
                        }
                    }
                    // Do nothing if the first number is non-zero
                    _ => {}
                }

                match new_rows[row_idx][col_idx].value {
                    // This time, if it's 0, when know there is no other non-zero-starting column
                    0 => {}
                    // If non-zero value, use this value to reduce other rows
                    _ => {
                        // Scale the current row by its first element
                        let scale = new_rows[row_idx][col_idx];
                        for scale_col_idx in col_idx..n_cols {
                            new_rows[row_idx][scale_col_idx] =
                                new_rows[row_idx][scale_col_idx] / scale;
                        }
                        // zero all other columns
                        for other_row_idx in 0..n_rows {
                            if other_row_idx == row_idx {
                                continue;
                            }
                            if new_rows[other_row_idx][col_idx].value != 0 {
                                // A leading non-zero element exists, scale the current row
                                // accordingly and subtract it from the lower row to zero leading value
                                let scale = new_rows[other_row_idx][col_idx];
                                for lower_col_idx in col_idx..n_cols {
                                    new_rows[other_row_idx][lower_col_idx] = new_rows
                                        [other_row_idx][lower_col_idx]
                                        - scale * new_rows[row_idx][lower_col_idx];
                                }
                            }
                        }
                        continue 'next_row;
                    }
                }
            }
        }

        Matrix::new(new_rows)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Convert numbers into strings
        let vals = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|element| format!("{}", element))
                    .collect_vec()
            })
            .collect_vec();

        // Determine widest string's len()
        let max_len = vals
            .iter()
            .flat_map(|row| row.iter().map(|element| element.len()))
            .max()
            .unwrap_or(0);

        // Left pad elements and collect into space separated columns of LF separated rows
        let output = vals
            .iter()
            .map(|row| {
                row.iter()
                    .map(|element| format!("{:>width$}", element, width = max_len))
                    .collect_vec()
                    .join(" ")
            })
            .collect_vec()
            .join("\n");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::finite_field::GFElement;
    use itertools::Itertools;

    #[test]
    fn test_matrix_display() {
        // Construct small matrix
        //  0 1
        //  1 2
        let rows = (0..=1)
            .map(|row_idx| {
                (0..=1)
                    .map(|col_idx| GFElement::new(row_idx + col_idx, 3))
                    .collect_vec()
            })
            .collect_vec();
        let matrix = Matrix::new(rows);
        assert_eq!(format!("{}", matrix), "0 1\n1 2");
    }

    #[test]
    fn test_rref() {
        // Construct small matrix
        //  0 1
        //  1 2
        let rows = (0..=1)
            .map(|row_idx| {
                (0..=1)
                    .map(|col_idx| GFElement::new(row_idx + col_idx, 3))
                    .collect_vec()
            })
            .collect_vec();
        let matrix = Matrix::new(rows);
        // Verify reduced row_echelon_form is correct
        let matrix_rref = matrix.to_rref();
        assert_eq!(format!("{}", matrix_rref), "1 0\n0 1");
        // Verify that rref form stays
        assert_eq!(format!("{}", matrix_rref.to_rref()), "1 0\n0 1");
    }

    #[test]
    fn test_rref_larger_matrix() {
        // Construct a 3x4 matrix
        //  1 2 3 4
        //  0 1 2 3
        //  1 1 1 1
        let rows = vec![
            vec![
                GFElement::new(1, 5),
                GFElement::new(2, 5),
                GFElement::new(3, 5),
                GFElement::new(4, 5),
            ],
            vec![
                GFElement::new(0, 5),
                GFElement::new(1, 5),
                GFElement::new(2, 5),
                GFElement::new(3, 5),
            ],
            vec![
                GFElement::new(1, 5),
                GFElement::new(1, 5),
                GFElement::new(1, 5),
                GFElement::new(1, 5),
            ],
        ];
        let matrix = Matrix::new(rows);
        let matrix_rref = matrix.to_rref();

        // Expected RREF:
        //  1 0 4 3
        //  0 1 2 3
        //  0 0 0 0
        assert_eq!(format!("{}", matrix_rref), "1 0 4 3\n0 1 2 3\n0 0 0 0");
        // Verify that rref form stays
        assert_eq!(
            format!("{}", matrix_rref.to_rref()),
            format!("{}", matrix_rref)
        );
    }
}
