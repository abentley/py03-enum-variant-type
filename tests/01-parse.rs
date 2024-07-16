use enum_variant_type::EnumVariantType;
use pyo3::prelude::*;
use pyo3_enum_variant_type::EvtPyclass;
use pyo3::exceptions::PyTypeError;

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

impl<'source> FromPyObject<'source> for Foo {
    // Required method
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if false {}
        else if let Ok(x) = FromPyObject::extract(ob) {
            return Ok(Blue::into(x));
        }
        else if let Ok(x) = FromPyObject::extract(ob) {
            return Ok(Green::into(x));
        }
        Err(PyErr::new::<PyTypeError, _>("Cannot convert to Foo"))
    }
}

fn main() {
    let _x: Foo = Blue { num: 3 }.into();
}
