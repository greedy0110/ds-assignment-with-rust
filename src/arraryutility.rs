// array가 아니고, Vec에 대한 동작들이다.

fn findmax(v: &Vec<i32>, s_index: usize, e_index: usize) -> i32 {
    assert!(!v.is_empty());

    let mut max = v[s_index];
    for value in &v[s_index..e_index] {
        if *value > max {
            max = *value;
        }
    }
    max
}

fn findmin(v: &Vec<i32>, i: usize, j: usize) -> i32 {
    assert!(!v.is_empty());

    let mut min = v[i];
    for value in &v[i..j] {
        if *value < min {
            min = *value;
        }
    }
    min
}

fn findminpos(v: &Vec<i32>, i: usize, j: usize) -> usize {
    assert!(!v.is_empty());

    let mut minindex = 0;
    let mut min = v[0];
    for (index, value) in v[i..j].iter().enumerate() {
        if *value < min {
            min = *value;
            minindex = index;
        }
    }
    i + minindex
}

fn findmaxpos(v: &Vec<i32>, i: usize, j: usize) -> usize {
    assert!(!v.is_empty());

    let mut maxindex = 0;
    let mut max = v[0];
    for (index, value) in v[i..j].iter().enumerate() {
        if *value > max {
            max = *value;
            maxindex = index;
        }
    }
    i + maxindex
}

fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
    assert!(!v.is_empty());

    let temp = v[i];
    v[i] = v[j];
    v[j] = temp;
}

fn shift_right(v: &mut Vec<i32>, i: usize, j: usize) {
    assert!(!v.is_empty());

    for k in (i + 1..j).rev() {
        v[k] = v[k - 1];
    }
}

fn shift_left(v: &mut Vec<i32>, i: usize, j: usize) {
    assert!(!v.is_empty());

    for k in i..j - 1 {
        v[k] = v[k + 1];
    }
}

use rand::{Rng, thread_rng};

fn create_random_array(size: usize, min: i32, max: i32) -> Vec<i32> {
    let mut v = Vec::with_capacity(size);
    let mut rng = thread_rng();
    for i in 0..size {
        v.push(rng.gen_range(min..max));
    }
    v
}

fn copy_array(v: &Vec<i32>) -> Vec<i32> {
    let mut a = Vec::with_capacity(v.len());
    for elem in v {
        a.push(*elem);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_array() {
        let a = vec![1, 2, 3, 4, 5];
        let b = copy_array(&a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_create_random_array() {
        // random 어떻게 테스트하지?
        let a: Vec<i32> = create_random_array(100, 1, 10);

        println!("{:?}", a);
        assert_eq!(a.len(), 100);
        assert!(findmax(&a, 0, 100) <= 10);
        assert!(findmin(&a, 0, 100) >= 1);
    }

    #[test]
    fn test_shift_left() {
        let mut a = vec![1, 2, 3, 4, 5];
        shift_left(&mut a, 1, 5);
        assert_eq!(a, vec![1, 3, 4, 5, 5]);
    }

    #[test]
    fn test_shift_right() {
        let mut a = vec![1, 2, 3, 4, 5];
        shift_right(&mut a, 1, 5);
        assert_eq!(a, vec![1, 2, 2, 3, 4]);
    }

    #[test]
    fn test_findmax() {
        let a = vec![1, 5, 3, 7, 8];
        assert_eq!(findmax(&a, 0, 3), 5);
        assert_eq!(findmax(&a, 0, 5), 8);
    }

    #[test]
    fn test_findmaxpos() {
        let a = vec![1, 5, 3, 7, 3];
        assert_eq!(findmaxpos(&a, 0, 3), 1);
        assert_eq!(findmaxpos(&a, 2, 5), 3);
    }

    #[test]
    fn test_findmin() {
        let a = vec![1, 5, 3, 7, 8];
        assert_eq!(findmin(&a, 1, 3), 3);
        assert_eq!(findmin(&a, 0, 5), 1);
    }

    #[test]
    fn test_findminpos() {
        let a = vec![1, 5, 3, 7, 5];
        assert_eq!(findminpos(&a, 0, 3), 0);
        assert_eq!(findminpos(&a, 2, 5), 2);
    }

    #[test]
    fn test_swap() {
        let mut a = vec![1, 5, 3, 7, 5];
        swap(&mut a, 2, 3);
        assert_eq!(a, vec![1, 5, 7, 3, 5]);
    }
}
