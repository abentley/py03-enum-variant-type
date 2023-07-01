use py_enum_macro::EvtPyclass;
use pyo3::prelude::*;
use enum_variant_type::EnumVariantType;

#[derive(Debug, EvtPyclass, EnumVariantType)]
enum Foo {
    #[evt(pyclass)]
    Blue{num: i32},
    #[evt(pyclass)]
    Green{num: u32},
}

fn main() {
}
