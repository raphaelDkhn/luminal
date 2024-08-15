use luminal::{prelude::*, tests::assert_close};

use crate::compiler::CairoCompiler;

#[test]
fn test_log2() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let mut b = a.log2().retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);
    cx.execute();

    assert_close(&b.data(), &[0.0, 1.0, 1.5849625, 2.0])
}

#[test]
fn test_exp2() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let mut b = a.exp2().retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);
    cx.execute();

    assert_close(&b.data(), &[2., 4., 8., 16.])
}
