use crate::ops::reduce::ReduceOpMetadata;

pub(crate) fn precompute_reduce_op_metadata(
    input_shape: &[usize],
    axis: usize,
) -> ReduceOpMetadata {
    let input_rank = input_shape.len();
    assert!(axis < input_rank, "Axis out of bounds");

    let output_shape: Vec<usize> = input_shape
        .iter()
        .enumerate()
        .filter_map(|(i, &dim)| if i != axis { Some(dim) } else { None })
        .collect();

    let output_size: usize = output_shape.iter().product();
    let mut output_indices = Vec::with_capacity(input_shape.iter().product());

    let mut current_index = vec![0; input_rank];
    let mut output_index = 0;

    loop {
        output_indices.push(output_index);

        for i in (0..input_rank).rev() {
            current_index[i] += 1;
            if current_index[i] < input_shape[i] {
                break;
            }
            current_index[i] = 0;
        }

        if current_index.iter().all(|&x| x == 0) {
            break;
        }

        output_index = 0;
        let mut multiplier = 1;
        for i in (0..input_rank).rev() {
            if i != axis {
                output_index += current_index[i] * multiplier;
                multiplier *= input_shape[i];
            }
        }
    }

    ReduceOpMetadata {
        output_indices,
        output_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1d_tensor() {
        let input_shape = vec![5];
        let axis = 0;
        let res = precompute_reduce_op_metadata(&input_shape, axis);
        assert_eq!(res.output_size, 1);
        assert_eq!(res.output_indices, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_2d_tensor() {
        let input_shape = vec![2, 3];
        let axis = 1;
        let res = precompute_reduce_op_metadata(&input_shape, axis);
        assert_eq!(res.output_size, 2);
        assert_eq!(res.output_indices, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_3d_tensor_axis0() {
        let input_shape = vec![2, 2, 2];
        let axis = 0;
        let res = precompute_reduce_op_metadata(&input_shape, axis);
        assert_eq!(res.output_size, 4);
        assert_eq!(res.output_indices, vec![0, 1, 2, 3, 0, 1, 2, 3]);
    }

    #[test]
    fn test_3d_tensor_axis1() {
        let input_shape = vec![2, 3, 2];
        let axis = 1;
        let res = precompute_reduce_op_metadata(&input_shape, axis);
        assert_eq!(res.output_size, 4);
        assert_eq!(res.output_indices, vec![0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3]);
    }

    #[test]
    fn test_4d_tensor() {
        let input_shape = vec![2, 2, 3, 2];
        let axis = 2;
        let res = precompute_reduce_op_metadata(&input_shape, axis);
        assert_eq!(res.output_size, 8);
        assert_eq!(
            res.output_indices,
            vec![0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 4, 5, 4, 5, 4, 5, 6, 7, 6, 7, 6, 7]
        );
    }
}
