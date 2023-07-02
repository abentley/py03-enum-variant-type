use enum_variant_type::EnumVariantType;
use pyo3_enum_variant_type::EvtPyclass;
use pyo3::prelude::*;

#[derive(Debug, EvtPyclass, EnumVariantType)]
enum Foo {
    #[evt(pyclass)]
    Blue { num: i32 },
    #[evt(pyclass)]
    Green { num: u32 },
}

#[pymodule]
fn enum_to_py(_py: Python, m: &PyModule) -> PyResult<()> {
    Foo::add_variant_structs(m)?;
    Ok(())
}
fn main() {}
