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

    pub fn is_solvable(&self) -> bool {
        // Determine whether the puzzle corresponding to the matrix is solvable.
        // It will be assumed that the right most column is the target vector of
        // the augmented matrix

        let matrix_rref = self.to_rref();

        // First, check whether any row is unsolvable
        if matrix_rref.is_any_row_unsolvable() {
            return false;
        }

        // If no row is unsolvable and every row has a pivot, the puzzle is
        // solvable
        matrix_rref.unaugmented_matrix().every_row_has_a_pivot()
    }

    fn unaugmented_matrix(&self) -> Self {
        // Create a new matrix without the last column (the augmentation)
        let row_len = self
            .rows
            .first()
            .expect("Matrix should have at least one row")
            .len();
        let rows = self
            .rows
            .iter()
            .map(|row| row.iter().take(row_len - 1).copied().collect_vec())
            .collect_vec();

        Self::new(rows)
    }

    fn every_row_has_a_pivot(&self) -> bool {
        // Check whether every row has a pivot (leading 1 in coefficient part)
        // An all-zeros row is considered to have a pivot
        // A non-zero row has a pivot if the first non-zero element in the coefficient
        // part (excluding the last column) is 1
        self.rows.iter().all(|row| {
            row.iter().all(|x| x.value == 0)
                || row
                    .iter()
                    .take(row.len())
                    .find(|x| x.value != 0)
                    .map_or(false, |x| x.value == 1)
        })
    }

    fn every_column_has_a_pivot(&self) -> bool {
        // Transpose the matrix and check whether every row has a pivot
        // The augmentation of the matrix is ignored

        self.transpose().every_row_has_a_pivot()
    }

    fn transpose(&self) -> Self {
        // Return a new transposed matrix

        let rows = (0..self
            .rows
            .first()
            .expect("Matrix should have one row at least")
            .len())
            .map(|col_idx| {
                (0..self.rows.len())
                    .map(|row_idx| {
                        self.rows[col_idx][row_idx] // Transposition occurs here
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self::new(rows)
    }

    fn is_any_row_unsolvable(&self) -> bool {
        // Any row of the form (0,0,0,...,k) is unsolvable for k<>0.
        // This would correspond to a non zero value being the result of a sum
        // of values multiplied by 0.
        self.rows.iter().any(|row| {
            row.iter().take(row.len() - 1).all(|x| x.value == 0)
                && row.last().map_or(false, |x| x.value != 0)
        })
    }

    pub fn solution(&self) -> Option<Vec<GFElement>> {
        // If the Puzzle is solvable, return the last column of the RREF form matrix
        // NOTE: The matrix is turned to RREF format multiple times, once in is_solvable
        // and once explicitly after. This is to not have to verify if it is in RREF form.
        if !self.is_solvable() {
            return None;
        }

        let augmentation = self
            .to_rref()
            .rows
            .iter()
            .map(|row| row.iter().copied().last().expect("Empty row not expected"))
            .collect_vec();
        Some(augmentation)
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
    use rstest::rstest;

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

    #[test]
    fn test_solvable() {
        // Test whether a matrix is marked as solvable
        let rows = vec![
            vec![
                GFElement::new(1, 2),
                GFElement::new(0, 2),
                GFElement::new(1, 2),
            ],
            vec![
                GFElement::new(0, 2),
                GFElement::new(1, 2),
                GFElement::new(1, 2),
            ],
            vec![
                GFElement::new(0, 2),
                GFElement::new(0, 2),
                GFElement::new(0, 2),
            ],
        ];
        let matrix = Matrix::new(rows);
        assert_eq!(matrix.to_rref().is_solvable(), true);
    }

    #[test]
    fn test_unsolvable() {
        // Test whether a matrix is marked as unsolvable
        let rows = vec![
            vec![
                GFElement::new(1, 2),
                GFElement::new(0, 2),
                GFElement::new(1, 2),
            ],
            vec![
                GFElement::new(0, 2),
                GFElement::new(1, 2),
                GFElement::new(1, 2),
            ],
            vec![
                GFElement::new(0, 2),
                GFElement::new(0, 2),
                // Last "light" is 1 but no switches influence it
                GFElement::new(1, 2),
            ],
        ];
        let matrix = Matrix::new(rows);
        assert_eq!(matrix.to_rref().is_solvable(), false);
    }

    #[rstest]
    #[case(vec![vec![GFElement::new(0,2),GFElement::new(0,2),GFElement::new(1,2)]], true)]
    #[case(vec![vec![GFElement::new(0,2),GFElement::new(0,2),GFElement::new(0,2)]], false)]
    #[case(vec![vec![GFElement::new(1,2),GFElement::new(0,2),GFElement::new(1,2)]], false)]
    fn test_any_row_unsolvable(#[case] rows: Vec<Vec<GFElement>>, #[case] expected: bool) {
        assert_eq!(
            Matrix::new(rows).to_rref().is_any_row_unsolvable(),
            expected
        );
    }

    #[rstest]
    #[case(vec![vec![GFElement::new(0,2),GFElement::new(0,2)]], true)]
    #[case(vec![vec![GFElement::new(0,2),GFElement::new(1,2)]], true)]
    #[case(vec![vec![GFElement::new(2,3),GFElement::new(0,3)]], false)]
    #[case(vec![vec![GFElement::new(0,3),GFElement::new(2,3)]], false)]
    #[case(vec![
        vec![GFElement::new(0,3),GFElement::new(1,3)],
        vec![GFElement::new(0,3),GFElement::new(2,3)]
        ], false)]
    fn test_every_row_has_a_pivot(#[case] rows: Vec<Vec<GFElement>>, #[case] expected: bool) {
        assert_eq!(Matrix::new(rows).every_row_has_a_pivot(), expected);
    }

    #[rstest]
    #[case(vec![
        vec![GFElement::new(1,2),GFElement::new(0,2)],
        vec![GFElement::new(0,2),GFElement::new(1,2)],
        ], true)]
    #[case(vec![
        vec![GFElement::new(1,2),GFElement::new(0,2)],
        vec![GFElement::new(0,2),GFElement::new(0,2)],
        ], true)]
    #[case(vec![
        vec![GFElement::new(1,2),GFElement::new(1,2)],
        vec![GFElement::new(0,2),GFElement::new(0,2)],
        ], true)]
    #[case(vec![
        vec![GFElement::new(0,2),GFElement::new(0,2)],
        vec![GFElement::new(1,2),GFElement::new(1,2)],
        ], true)]
    fn test_every_column_has_a_pivot(#[case] rows: Vec<Vec<GFElement>>, #[case] expected: bool) {
        assert_eq!(Matrix::new(rows).every_column_has_a_pivot(), expected);
    }

    #[rstest]
    #[case(vec![
        vec![GFElement::new(1,2), GFElement::new(0,2), GFElement::new(1,2)],
        vec![GFElement::new(0,2), GFElement::new(1,2), GFElement::new(1,2)],
    ], vec![GFElement::new(1,2),GFElement::new(1,2)])]
    #[case(vec![
        vec![GFElement::new(2,7), GFElement::new(3,7), GFElement::new(5,7)],
        vec![GFElement::new(2,7), GFElement::new(6,7), GFElement::new(1,7)],
    ], vec![GFElement::new(1,7),GFElement::new(1,7)])]
    fn test_solution(#[case] rows: Vec<Vec<GFElement>>, #[case] expected: Vec<GFElement>) {
        assert_eq!(Matrix::new(rows).solution(), Some(expected));
    }
}
