use quote::quote;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = quote!{ pub struct DerivedStruct; };
    result.into()
}