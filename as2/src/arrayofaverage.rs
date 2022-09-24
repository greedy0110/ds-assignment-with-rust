type Matrix = Vec<Vec<f64>>;

/// ps[i] => [0, i) 까지의 합
/// ps[0] => 아무것도 더하지 않은 값
/// a [i,j) 범위 합은? ps[j] - ps[i]로 구할 수 있음
fn particial_sum(a: &Vec<f64>) -> Vec<f64> {
    let n = a.len();
    let mut ps = Vec::with_capacity(n + 1);
    ps.push(0.0);
    for i in 0..n {
        ps.push(ps[i] + a[i]);
    }
    assert_eq!(ps.len(), n + 1);
    ps
}

pub fn array_of_average(a: &Vec<f64>) -> Matrix {
    let n = a.len();
    let mut matrix: Matrix = vec![vec![0.0; n]; n];
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part00() {
        let ps = particial_sum(&vec![1.0, 2.0, 3.0]);
        assert_eq!(ps, vec![0.0, 1.0, 3.0, 6.0]);
        // [1,2) 구간합
        assert_eq!(ps[2] - ps[1], 2.0);
        // [0,2) 구간합
        assert_eq!(ps[2] - ps[0], 3.0);
        // [0,3) 구간합
        assert_eq!(ps[3] - ps[0], 6.0);
    }

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
