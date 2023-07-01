use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn no_op(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(EvtPyclass)]
pub fn evt_pyclass(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    r#"
impl IntoPy<PyObject> for OneOrTwoB {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            OneOrTwoB::ONE{..} => TryInto::<ONE>::try_into(self).unwrap().into_py(py),
            OneOrTwoB::TWO{..} => TryInto::<TWO>::try_into(self).unwrap().into_py(py),
        }
    }
}
"#.parse::<TokenStream>().expect("Generated unparseable shtuff")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
