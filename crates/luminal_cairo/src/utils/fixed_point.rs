const ONE: f32 = 4294967296.0;

pub(crate) fn fp_to_float(a: i64) -> f32 {
    a as f32 / ONE
}

pub(crate) fn from_float_to_fp(a: f32) -> i64 {
    (a * ONE) as i64
}
