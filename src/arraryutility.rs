// fn findMax<T>(v: &Vec<T>, s_index: usize, e_index: usize) -> T
// where
//     T: Ord, Clone, Copy
// {
//     let mut max = &v[s_index];
//     for value in &v[s_index..e_index] {
//         if value > max {
//             max = value;
//         }
//     }
//     max
// }

fn findmax(v: &Vec<i32>, s_index: usize, e_index: usize) -> i32 {
    let mut max = v[s_index];
    for value in &v[s_index..e_index] {
        if *value > max {
            max = *value;
        }
    }
    max
}

fn findmin(v: &Vec<i32>, i: usize, j: usize) -> i32 {
    let mut min = v[i];
    for value in &v[i..j] {
        if *value < min {
            min = *value;
        }
    }
    min
}

fn findminpos(v: &Vec<i32>, i: usize, j:usize) -> usize {
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

fn findmaxpos(v: &Vec<i32>, i: usize, j:usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
