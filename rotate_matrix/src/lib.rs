fn rotate_matrix(image: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let edge = image.len();
    let mut transposed = vec![vec![0; edge]; edge];
    for i in (0..edge).rev() {
        for j in 0..edge {
            transposed[j][edge - i - 1] = image[i][j];
        }
    }
    transposed
}

#[cfg(test)]
mod test_rotate_matrix {
    use super::*;

    #[test]
    fn is_ok() {
        assert_eq!(
            rotate_matrix(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3],]),
            vec![vec![3, 2, 1], vec![3, 2, 1], vec![3, 2, 1],]
        );
    }
}
