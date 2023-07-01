use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_attribute]
pub fn no_op(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(EvtPyclass)]
pub fn evt_pyclass(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let mut outstr = format!("impl IntoPy<PyObject> for {}", ast.ident);
    let Data::Enum(de) = ast.data else {
        panic!("Can only be used with enum.")
    };

    outstr.push_str(r#" {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
"#);
    for variant in de.variants {
    outstr.push_str(&format!(
        "            {}::{}{{..}} => TryInto::<{}>::try_into(self).unwrap().into_py(py),\n", ast.ident, variant.ident, variant.ident));
    }
    outstr.push_str(r#"        }
    }
}
"#);
    outstr.parse::<TokenStream>().expect("Generated unparseable shtuff")
}
