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

#[test]
fn test_sin() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let mut b = a.sin().retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);
    cx.execute();

    assert_close(&b.data(), &[0.84147098, 0.90929743, 0.14112001, -0.7568025])
}

#[test]
fn test_sqrt() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let mut b = a.sqrt().retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut b);
    cx.execute();

    assert_close(&b.data(), &[1., 1.41421356, 1.73205081, 2.])
}
