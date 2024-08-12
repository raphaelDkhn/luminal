use luminal::{
    prelude::*,
    tests::{self, assert_close},
};

use crate::{cairo_runner::CairoRunnerConfig, compiler::CairoCompiler};

#[test]
fn test_add() {
    let mut cx = Graph::new();
    let a = cx.tensor((2, 2)).set([[1.0, 2.0, 3.0, 4.0]]);
    let b = cx.tensor((2, 2)).set([[10.0, 20.0, 30.0, 40.0]]);

    // Actual operations
    let mut c = (a + b).retrieve();

    let compiled_programs =
        "/Users/raphaeldoukhan/Desktop/Giza/Frameworks/luminal/crates/luminal_cairo/compiled_cairo";

    let config = CairoRunnerConfig {
        trace_file: None,
        memory_file: None,
        proof_mode: false,
        air_public_input: None,
        air_private_input: None,
        cairo_pie_output: None,
        append_return_values: false,
    };

    let cairo_compiler = CairoCompiler::new(compiled_programs.into(), config);

    let _ = cx.compile(cairo_compiler, &mut c);
    cx.execute();

    // assert_close(&c.data(), &[11.0, 22.0, 33.0, 44.0])
}
