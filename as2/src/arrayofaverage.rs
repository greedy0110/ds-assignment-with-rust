type Matrix = Vec<Vec<f64>>;

fn array_of_average(a: &Vec<f64>) -> Matrix {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let a = vec![];
        let expected = vec![];
        let actual = array_of_average(&a);
        assert_matrix_equal(&actual, &expected)
    }

    #[test]
    fn test02() {
        let a = vec![0.0];
        let expected = vec![vec![0.0]];
        let actual = array_of_average(&a);
        assert_matrix_equal(&actual, &expected)
    }

    #[test]
    fn test03() {
        let a = vec![0.0, 1.0];
        let expected = vec![vec![0.0, 0.5], vec![0.0, 1.0]];
        let actual = array_of_average(&a);
        assert_matrix_equal(&actual, &expected)
    }

    #[test]
    fn test04() {
        let a = vec![0.0, 1.0, 2.0];
        let expected = vec![
            vec![0.0, 0.5, 1.0],
            vec![0.0, 1.0, 1.5],
            vec![0.0, 0.0, 2.0],
        ];
        let actual = array_of_average(&a);
        assert_matrix_equal(&actual, &expected)
    }

    fn assert_matrix_equal(actual: &Matrix, expected: &Matrix) {
        assert_eq!(actual.len(), expected.len());
        for i in 0..actual.len() {
            assert_eq!(actual[i].len(), expected[i].len());
            for j in 0..actual[i].len() {
                let actual = actual[i][j];
                let expected = expected[i][j];
                let delta = 1e-10;
                assert!((actual - expected).abs() <= delta);
            }
        }
    }
}
