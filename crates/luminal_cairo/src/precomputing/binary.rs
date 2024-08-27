use luminal::shape::ShapeTracker;

pub(crate) fn precompute_binary_op_metadata(
    lhs_shape: &ShapeTracker,
    rhs_shape: &ShapeTracker,
) -> (Vec<usize>, Vec<usize>) {
    let lhs_dims = lhs_shape.shape_usize();
    let rhs_dims = rhs_shape.shape_usize();
    
    let broadcast_shape = broadcast_shapes(&lhs_dims, &rhs_dims);
    let lhs_indices = compute_indices(&lhs_dims, &broadcast_shape, lhs_shape.fake.as_slice());
    let rhs_indices = compute_indices(&rhs_dims, &broadcast_shape, rhs_shape.fake.as_slice());
    
    (lhs_indices, rhs_indices)
}

fn broadcast_shapes(shape1: &[usize], shape2: &[usize]) -> Vec<usize> {
    let max_len = shape1.len().max(shape2.len());
    let mut result = Vec::with_capacity(max_len);
    
    for i in 0..max_len {
        let dim1 = shape1.get(shape1.len().saturating_sub(max_len - i)).copied().unwrap_or(1);
        let dim2 = shape2.get(shape2.len().saturating_sub(max_len - i)).copied().unwrap_or(1);
        result.push(dim1.max(dim2));
    }
    
    result
}

fn compute_indices(shape: &[usize], broadcast_shape: &[usize], fake: &[bool]) -> Vec<usize> {
    let mut indices = Vec::with_capacity(broadcast_shape.iter().product());
    let mut current_index = vec![0; broadcast_shape.len()];
    
    loop {
        let mut idx = 0;
        for (i, &dim) in shape.iter().enumerate() {
            if !fake[i] {
                idx = idx * dim + current_index[broadcast_shape.len() - shape.len() + i] % dim;
            }
        }
        indices.push(idx);
        
        // Increment current_index
        for i in (0..broadcast_shape.len()).rev() {
            current_index[i] += 1;
            if current_index[i] < broadcast_shape[i] {
                break;
            }
            current_index[i] = 0;
            if i == 0 {
                return indices;
            }
        }
    }
}