use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(command))]
struct RootConfig {
    name: String,
}

#[derive(FromField)]
#[darling(attributes(command))]
struct ArgumentConfig {
    name: String,
    value: Ident,
}

#[proc_macro_derive(Command, attributes(command, argument))]
pub fn derive_command(item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);

    let root_config = match RootConfig::from_derive_input(&derive_input) {
        Ok(root_config) => root_config,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let root_name = root_config.name;

    let name = derive_input.ident;

    let mut options = Vec::new();
    let mut subcommands = Vec::new();

    match derive_input.data {
        syn::Data::Struct(data) => {
            for field in data.fields {
                let argument_config = match ArgumentConfig::from_field(&field) {
                    Ok(argument_config) => argument_config,
                    Err(e) => return TokenStream::from(e.write_errors()),
                };
                let name = argument_config.name;
                let field_type = field.ty;

                options.push(quote!(
                    bot_any_cal::CommandOption {
                        name: #name,
                        description: None,
                        value: <
                            #field_type as bot_any_cal::GetCommandOptionValueKind
                        >.get_command_option_value_kind(),
                    }
                ));
            }
        }
        syn::Data::Enum(data) => {
            for variant in data.variants {
                if variant.

                subcommands.push(quote!());
            }
        }
        syn::Data::Union(data) => {
            return TokenStream::from(
                syn::Error::new_spanned(data.union_token, "Cannot derive Command for union")
                    .into_compile_error(),
            )
        }
    };

    quote!(
        impl Command for #name {
            fn spec() -> bot_any_cal::CommandSpec {
                bot_any_cal::CommandSpec {
                    name: #root_name,
                    description: None,
                    options: vec![#(#options,)*],
                    subcommands: vec![],
                }
            }
        }
    )
    .into()
}
