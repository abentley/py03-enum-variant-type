use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/**
 * Generate an implementation of `IntoPy<PyObject>` for an enum.  It assumes that each variant has a
 * corresponding struct, as generated by `EnumVariantType` derive macro.  Further, that struct must
 * have a `TryFrom` / `TryInto` implementation for the enum (as generated by EVT), and its own
 * `IntoPy<PyObject>` implementation (as generated by pyop3's pyclass macro.
 */
#[proc_macro_derive(EvtPyclass)]
pub fn evt_pyclass(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let Data::Enum(ref de) = ast.data else {
        panic!("Can only be used with enum.")
    };
    let ident = ast.ident;
    let arms = de
        .variants
        .clone()
        .into_iter()
        .map(|v| v.ident)
        .collect::<Vec<_>>();
    quote! {
        impl IntoPy<PyObject> for #ident {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    #(#ident::#arms{..} => TryInto::<#arms>::try_into(self).unwrap().into_py(py),)*
                }
            }
        }
        impl #ident {
            fn add_variant_structs(m: &PyModule) -> PyResult<()>{
                #(m.add_class::<#arms>()?;)*
                Ok(())
            }
        }
        impl<'source> FromPyObject<'source> for #ident {
            // Required method
            fn extract(ob: &'source PyAny) -> PyResult<Self> {
                if false {}
                #(else if let Ok(x) = FromPyObject::extract(ob) {
                    return Ok(#arms::into(x));
                })*
                Err(PyErr::new::<PyTypeError, _>("Cannot convert to Foo"))
            }
        }
    }
    .into()
}
