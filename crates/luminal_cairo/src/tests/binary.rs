use luminal::{prelude::*, tests::assert_close};

use crate::compiler::CairoCompiler;

#[test]
fn test_add() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let b = cx.tensor((2, 2)).set([[10.0, 20.0, 30.0, 40.0]]);
    let c = a + b;
    let bias = cx.tensor((2, 2)).set([[1.0, 1.0, 1.0, 1.0]]);
    let mut d = (c + bias).retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut d);
    cx.execute();

    assert_eq!(d.shape.shape_usize(), vec![2, 2]);
    assert_close(&d.data(), &[12.0, 23.0, 34.0, 45.0])
}

#[test]
fn test_mul() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let b = cx.tensor((2, 2)).set([[10.0, 20.0, 30.0, 40.0]]);
    let c = a * b;
    let bias = cx.tensor((2, 2)).set([[2.0, 2.0, 2.0, 2.0]]);
    let mut d = (c * bias).retrieve();

    let cairo_compiler = CairoCompiler::default();

    let _ = cx.compile(cairo_compiler, &mut d);
    cx.execute();

    assert_eq!(d.shape.shape_usize(), vec![2, 2]);
    assert_close(&d.data(), &[20.0, 80.0, 180.0, 320.0])
}
