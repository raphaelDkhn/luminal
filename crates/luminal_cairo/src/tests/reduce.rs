use luminal::{prelude::*, tests::assert_close};

use crate::compiler::CairoCompiler;

// #[test]
// fn test_sum_reduce_1d() {
//     let mut cx = Graph::new();
//     let a = cx.tensor(4).set([[1.0, 2.0, 3.0, 4.0]]);
//     let mut b = a.sum_reduce(0).retrieve();

//     let cairo_compiler = CairoCompiler::default();

//     let _ = cx.compile(cairo_compiler, &mut b);
//     cx.execute();

//     assert_close(&b.data(), &[10.0])
// }

#[test]
fn test_sum_reduce_2d() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let mut b = a.sum_reduce(1).retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);
    cx.execute();

    assert_close(&b.data(), &[3.0, 7.0])
}
