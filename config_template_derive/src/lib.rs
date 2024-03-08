
use proc_macro::TokenStream;
use syn;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(ConfigTemplate, attributes(default_value))]
pub fn derive_config_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{}", input.ident);

    match input.data {
        syn::Data::Struct(ss) => {
            match ss.fields {
                syn::Fields::Named(named) => {
                    for f in named.named {
                        println!("field: {}", f.ident.unwrap().to_string());
                    }
                },
                syn::Fields::Unnamed(_) => (),
                syn::Fields::Unit => (),
            }

            ()
        },
        syn::Data::Enum(_) => (),
        syn::Data::Union(_) => (),
    }

    for a in input.attrs {
        println!("  {}", a.path().get_ident().unwrap().to_string());
    }

    TokenStream::new()

    // TokenStream::from(input.into_token_stream())
}
