use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[derive(FromDeriveInput, FromVariant)]
#[darling(attributes(command))]
struct CommandConfig {
    name: Option<String>,
    description: Option<String>,

    #[darling(rename = "self")]
    for_self: Option<bool>,
}

#[derive(FromField)]
#[darling(attributes(argument))]
struct ArgumentConfig {
    name: String,
    description: String,
}

#[proc_macro_derive(Command, attributes(command, argument))]
pub fn derive_command(item: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(item as DeriveInput);

    let root_command_config = match CommandConfig::from_derive_input(&derive_input) {
        Ok(root_config) => root_config,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let root_command_name = match root_command_config.name {
        Some(command_name) => command_name,
        None => {
            return TokenStream::from(
                syn::Error::new_spanned(derive_input.ident, "#[command] attribute requires a name")
                    .into_compile_error(),
            )
        }
    };
    let root_command_description = match root_command_config.description {
        Some(command_description) => command_description,
        None => {
            return TokenStream::from(
                syn::Error::new_spanned(derive_input.ident, "#[command] attribute requires a description")
                    .into_compile_error(),
            )
        }
    };

    let name = derive_input.ident;

    let mut option_idents = Vec::new();
    let mut option_names = Vec::new();
    let mut option_descriptions = Vec::new();
    let mut option_types = Vec::new();

    let mut subcommands = Vec::new();
    let mut subcommands_named_fields_match_arms = Vec::new();
    let mut subcommand_match_arms = Vec::new();

    let mut self_discovered = Vec::new();

    match derive_input.data {
        syn::Data::Struct(data) => {
            self_discovered.push(quote! {
                #name
            });
            for field in data.fields {
                let argument_config = match ArgumentConfig::from_field(&field) {
                    Ok(argument_config) => argument_config,
                    Err(e) => return TokenStream::from(e.write_errors()),
                };
                let field_ident = match field.ident {
                    Some(name) => name,
                    None => {
                        return TokenStream::from(
                            syn::Error::new_spanned(field, "enum variant field must have a name")
                                .into_compile_error(),
                        )
                    }
                };

                option_idents.push(field_ident);
                option_names.push(argument_config.name);
                option_descriptions.push(argument_config.description);
                option_types.push(field.ty);
            }
        }
        syn::Data::Enum(data) => {
            for variant in data.variants {
                let command_config = CommandConfig::from_variant(&variant);

                match variant.fields {
                    Fields::Named(fields) => {
                        let mut inner_option_idents = Vec::new();
                        let mut inner_option_names = Vec::new();
                        let mut inner_option_descriptions = Vec::new();
                        let mut inner_option_types = Vec::new();
                        for field in fields.named {
                            let argument_config = match ArgumentConfig::from_field(&field) {
                                Ok(argument_config) => argument_config,
                                Err(e) => return TokenStream::from(e.write_errors()),
                            };
                            let ident = match field.ident {
                                Some(name) => name,
                                None => {
                                    return TokenStream::from(
                                        syn::Error::new_spanned(
                                            field,
                                            "enum variant field must have a name",
                                        )
                                        .into_compile_error(),
                                    )
                                }
                            };
                            inner_option_idents.push(ident);
                            inner_option_names.push(argument_config.name);
                            inner_option_descriptions.push(argument_config.description);
                            inner_option_types.push(field.ty);
                        }

                        let command_config = match command_config {
                            Ok(config) => config,
                            Err(e) => return TokenStream::from(e.write_errors()),
                        };

                        let variant_name = variant.ident;

                        if command_config.for_self.unwrap_or(false) {
                            self_discovered.push(quote! {
                                #name::#variant_name
                            });
                            option_idents.extend(inner_option_idents);
                            option_names.extend(inner_option_names);
                            option_descriptions.extend(inner_option_descriptions);
                            option_types.extend(inner_option_types);
                        } else {
                            let command_name = match command_config.name {
                                Some(command_name) => command_name,
                                None => {
                                    return TokenStream::from(
                                        syn::Error::new_spanned(
                                            variant_name,
                                            "#[command] attribute requires a name",
                                        )
                                        .into_compile_error(),
                                    )
                                }
                            };
                            let command_description = match command_config.description {
                                Some(command_description) => command_description,
                                None => {
                                    return TokenStream::from(
                                        syn::Error::new_spanned(
                                            variant_name,
                                            "#[command] attribute requires a description",
                                        )
                                        .into_compile_error(),
                                    )
                                }
                            };
                            subcommands_named_fields_match_arms.push(quote! {
                                #command_name => {
                                    #(
                                        let mut #inner_option_idents: ::std::option::Option<#inner_option_types> = <#inner_option_types as ::bot_any_cal::CommandOptionValueTy>::default();
                                    )*
            
                                    for argument in arguments {
                                        match argument.name.as_str() {
                                            #(
                                                #inner_option_names => {
                                                    #inner_option_idents = ::std::option::Option::Some(argument.value.clone().try_into().ok()?);
                                                }
                                            )*
                                            _ => {
                                                continue;
                                            }
                                        }
                                    }
            
                                    match (#(#inner_option_idents),*) {
                                        (#(::std::option::Option::Some(#inner_option_idents)),*) => {
                                            ::std::option::Option::Some(#name::#variant_name {
                                                #(#inner_option_idents),*
                                            })
                                        }
                                        _ => ::std::option::Option::None,
                                    }
                                }
                            });
                            subcommands.push(quote! {
                                ::bot_any_cal::CommandSpec {
                                    name: #command_name,
                                    description: #command_description,
                                    options: vec![#(::bot_any_cal::CommandOption {
                                        name: #inner_option_names,
                                        description: #inner_option_descriptions,
                                        value: <#inner_option_types as ::bot_any_cal::CommandOptionValueTy>::spec_kind(),
                                    }),*],
                                    subcommands: ::std::vec::Vec::new(),
                                }
                            });
                        }
                    }
                    Fields::Unnamed(fields) => {
                        if fields.unnamed.len() != 1 {
                            return TokenStream::from(
                                syn::Error::new_spanned(
                                    fields,
                                    "Unnamed enum variant must have one field",
                                )
                                .into_compile_error(),
                            );
                        }
                        let variant_name = variant.ident;
                        let ty = &fields.unnamed[0].ty;
                        subcommand_match_arms.push(quote! {
                            <#ty as ::bot_any_cal::Command>::NAME => {
                                <#ty as ::bot_any_cal::Command>::parse(rest).map(#name::#variant_name)
                            }
                        });
                        subcommands.push(quote! {
                            <#ty as ::bot_any_cal::Command>::spec()
                        });
                    }
                    Fields::Unit => {
                        let command_config = match command_config {
                            Ok(root_config) => root_config,
                            Err(e) => return TokenStream::from(e.write_errors()),
                        };

                        let command_name = match command_config.name {
                            Some(command_name) => command_name,
                            None => {
                                return TokenStream::from(
                                    syn::Error::new_spanned(
                                        variant.ident,
                                        "#[command] attribute requires a name",
                                    )
                                    .into_compile_error(),
                                )
                            }
                        };
                        let command_description = match command_config.description {
                            Some(command_description) => command_description,
                            None => {
                                return TokenStream::from(
                                    syn::Error::new_spanned(
                                        variant.ident,
                                        "#[command] attribute requires a description",
                                    )
                                    .into_compile_error(),
                                )
                            }
                        };
                        subcommands.push(quote! {
                            ::bot_any_cal::CommandSpec {
                                name: #command_name,
                                description: #command_description,
                                options: ::std::vec::Vec::new(),
                                subcommands: ::std::vec::Vec::new(),
                            }
                        })
                    }
                };
            }
        }
        syn::Data::Union(data) => {
            return TokenStream::from(
                syn::Error::new_spanned(data.union_token, "Cannot derive Command for union")
                    .into_compile_error(),
            )
        }
    };


    if self_discovered.len() > 1 {
        return TokenStream::from(
            syn::Error::new_spanned(
                &self_discovered[0],
                "Cannot set #[command(self)] more than once",
            )
            .into_compile_error(),
        );
    }

    let self_arm = if let Some(self_token) = self_discovered.first() {
        Some(quote! {
            [::bot_any_cal::CommandPreflight::Execute(arguments)] => {
                #(
                    let mut #option_idents: ::std::option::Option<#option_types> = <#option_types as ::bot_any_cal::CommandOptionValueTy>::default();
                )*

                for argument in arguments {
                    match argument.name.as_str() {
                        #(
                            #option_names => {
                                #option_idents = ::std::option::Option::Some(argument.value.clone().try_into().ok()?);
                            }
                        )*
                        _ => {
                            continue;
                        }
                    }
                }

                match (#(#option_idents),*) {
                    (#(::std::option::Option::Some(#option_idents)),*) => {
                        ::std::option::Option::Some(#self_token {
                            #(#option_idents),*
                        })
                    }
                    _ => ::std::option::Option::None,
                }
            }
        })
    } else {
        None
    };

    quote! {
        impl Command for #name {
            const NAME: &'static str = #root_command_name;

            fn spec() -> ::bot_any_cal::CommandSpec {
                ::bot_any_cal::CommandSpec {
                    name: #root_command_name,
                    description: #root_command_description,
                    options: ::std::vec![#(
                        ::bot_any_cal::CommandOption {
                            name: #option_names,
                            description: #option_descriptions,
                            value: <#option_types as ::bot_any_cal::CommandOptionValueTy>::spec_kind(),
                        }),*],
                    subcommands: ::std::vec![#(#subcommands),*],
                }
            }
            fn parse(preflights: &[::bot_any_cal::CommandPreflight]) -> ::std::option::Option<Self> {
                match preflights {
                    [
                        ::bot_any_cal::CommandPreflight::Select(name),
                        execute @ ::bot_any_cal::CommandPreflight::Execute(arguments),
                        ..
                    ] => {
                        let rest = ::std::slice::from_ref(execute);
                        match name.as_str() {
                            #(#subcommands_named_fields_match_arms),*
                            #(#subcommand_match_arms),*
                            _ => ::std::option::Option::None,
                        }
                    }
                    [::bot_any_cal::CommandPreflight::Select(name), rest @ ..] => {
                        match name.as_str() {
                            #(#subcommand_match_arms),*
                            _ => ::std::option::Option::None,
                        }
                    }
                    #self_arm
                    _ => ::std::option::Option::None,
                }
            }
        }
    }
    .into()
}
