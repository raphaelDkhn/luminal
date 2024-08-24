use luminal::{prelude::*, tests::assert_close};
use luminal_nn::Linear;

use crate::compiler::CairoCompiler;

#[test]
fn test_linear() {
    let mut cx = Graph::new();

    // Initialize a linear layer with an input size of 4 and an output size of 5 with no bias
    let model = Linear::new(4, 5, false, &mut cx).initialize();

    model.weight.set([
        [0.1, 0.2, 0.3, 0.4],
        [0.5, 0.6, 0.7, 0.8],
        [0.9, 1.0, 1.1, 1.2],
        [1.3, 1.4, 1.5, 1.6],
        [1.7, 1.8, 1.9, 2.0],
    ]);

    let a = cx.tensor(4).set(vec![1., 2., 3., 4.]);
    let mut b = model.forward(a).retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);

    cx.execute();

    let expected_output: Vec<f32> = vec![11.0, 12.0, 13.0, 14.0, 15.0];

    assert_close(&b.data(), &expected_output);
}
