use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_attribute]
pub fn no_op(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(EvtPyclass)]
pub fn evt_pyclass(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let Data::Enum(ref de) = ast.data else {
        panic!("Can only be used with enum.")
    };
    let ident = ast.ident;
    let mut arms = vec![];
    for variant in &de.variants {
        arms.push(variant.ident.clone())
    }
    let x: TokenStream = quote! {
        impl IntoPy<PyObject> for #ident {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    #(#ident::#arms{..} => TryInto::<#arms>::try_into(self).unwrap().into_py(py),)*
                }
            }
        }
    }
    .into();
    eprintln!("{:?}", arms);
    eprintln!("!! {:#?} !!", x);
    x
}
