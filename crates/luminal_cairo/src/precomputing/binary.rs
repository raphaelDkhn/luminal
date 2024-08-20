use std::cmp::max;

fn broadcast_with(shape1: &[usize], shape2: &[usize]) -> Vec<usize> {
    let max_len = max(shape1.len(), shape2.len());
    let mut result = Vec::with_capacity(max_len);
    let mut iter1 = shape1.iter().rev();
    let mut iter2 = shape2.iter().rev();

    for _ in 0..max_len {
        let dim1 = iter1.next().copied().unwrap_or(1);
        let dim2 = iter2.next().copied().unwrap_or(1);
        result.push(max(dim1, dim2));
    }
    result.reverse();
    result
}

fn compute_strides(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![1; shape.len()];
    for i in (0..shape.len() - 1).rev() {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
    strides
}

fn compute_indices(shape: &[usize], broadcast_shape: &[usize]) -> Vec<usize> {
    let total_elements: usize = broadcast_shape.iter().product();
    let mut indices = Vec::with_capacity(total_elements);
    let strides = compute_strides(shape);
    let offset = broadcast_shape.len() - shape.len();

    let mut current_index = vec![0; broadcast_shape.len()];
    for _ in 0..total_elements {
        let mut idx = 0;
        for (i, (&dim, &stride)) in shape.iter().zip(&strides).enumerate() {
            let broadcast_dim = broadcast_shape[i + offset];
            idx += ((current_index[i + offset] % broadcast_dim) % dim) * stride;
        }
        indices.push(idx);

        // Increment current_index
        for i in (0..broadcast_shape.len()).rev() {
            current_index[i] += 1;
            if current_index[i] < broadcast_shape[i] {
                break;
            }
            current_index[i] = 0;
        }
    }
    indices
}

pub(crate) fn precompute_binary_op_metadata(
    lhs_shape: &[usize],
    rhs_shape: &[usize],
) -> (Vec<usize>, Vec<usize>) {
    let broadcast_shape = broadcast_with(lhs_shape, rhs_shape);
    let lhs_indices = compute_indices(lhs_shape, &broadcast_shape);
    let rhs_indices = compute_indices(rhs_shape, &broadcast_shape);
    (lhs_indices, rhs_indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_op_metadata_simple_broadcast() {
        let lhs_shape = vec![1, 5];
        let rhs_shape = vec![5];
        let (lhs_indices, rhs_indices) = precompute_binary_op_metadata(&lhs_shape, &rhs_shape);
        assert_eq!(lhs_indices, vec![0, 1, 2, 3, 4]);
        assert_eq!(rhs_indices, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_binary_op_metadata_3d_broadcast() {
        let lhs_shape = vec![3, 1, 2];
        let rhs_shape = vec![1, 4, 2];
        let (lhs_indices, rhs_indices) = precompute_binary_op_metadata(&lhs_shape, &rhs_shape);
        assert_eq!(
            lhs_indices,
            vec![0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5]
        );
        assert_eq!(
            rhs_indices,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]
        );
    }

    #[test]
    fn test_binary_op_metadata_complex_broadcast() {
        let lhs_shape = vec![8, 1, 5, 1];
        let rhs_shape = vec![7, 1, 1];
        let (lhs_indices, rhs_indices) = precompute_binary_op_metadata(&lhs_shape, &rhs_shape);
        assert_eq!(
            lhs_indices,
            vec![
                0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 0, 1, 2,
                3, 4, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 5,
                6, 7, 8, 9, 5, 6, 7, 8, 9, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 10, 11, 12, 13, 14,
                10, 11, 12, 13, 14, 10, 11, 12, 13, 14, 10, 11, 12, 13, 14, 10, 11, 12, 13, 14, 10,
                11, 12, 13, 14, 15, 16, 17, 18, 19, 15, 16, 17, 18, 19, 15, 16, 17, 18, 19, 15, 16,
                17, 18, 19, 15, 16, 17, 18, 19, 15, 16, 17, 18, 19, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 20, 21, 22, 23, 24, 20, 21, 22, 23, 24, 20, 21, 22, 23, 24, 20, 21, 22, 23,
                24, 20, 21, 22, 23, 24, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 25, 26, 27, 28, 29,
                25, 26, 27, 28, 29, 25, 26, 27, 28, 29, 25, 26, 27, 28, 29, 25, 26, 27, 28, 29, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 30, 31, 32, 33, 34, 30, 31, 32, 33, 34, 30, 31,
                32, 33, 34, 30, 31, 32, 33, 34, 30, 31, 32, 33, 34, 30, 31, 32, 33, 34, 35, 36, 37,
                38, 39, 35, 36, 37, 38, 39, 35, 36, 37, 38, 39, 35, 36, 37, 38, 39, 35, 36, 37, 38,
                39, 35, 36, 37, 38, 39, 35, 36, 37, 38, 39
            ]
        );
        assert_eq!(
            rhs_indices,
            vec![
                0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5,
                5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4,
                4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2,
                2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1,
                1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6,
                0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5,
                5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4,
                4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2,
                2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 1, 1,
                1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6
            ]
        );
    }
}
