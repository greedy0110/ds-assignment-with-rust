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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_findmax() {
        let a = vec![1, 5, 3];
        assert_eq!(findmax(&a, 0, 3), 5);
    }
}
