use enum_variant_type::EnumVariantType;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3_enum_variant_type::EvtPyclass;

#[derive(Debug, EvtPyclass, EnumVariantType)]
enum Foo {
    #[evt(pyclass(set_all), derive(FromPyObject))]
    Blue { num: i32 },
    #[evt(pyclass, derive(FromPyObject))]
    Green { num: u32 },
}

#[pymodule]
fn enum_to_py(_py: Python, m: &PyModule) -> PyResult<()> {
    Foo::add_variant_structs(m)?;
    Ok(())
}

fn main() {
    let _x: Foo = Blue { num: 3 }.into();
}
