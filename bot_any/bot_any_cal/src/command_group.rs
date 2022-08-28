#[macro_export]
macro_rules! command_group {
    ($name:ident { $($children:ident,)* }) => {
        pub enum $name {
            $($children($children)),*
        }

        impl $name {
            pub fn contains(name: &str) -> bool {
                ::std::matches!(name, $(<$children as ::bot_any_cal::Command>::NAME)|*)
            }

            pub fn children_specs() -> std::vec::Vec<::bot_any_cal::CommandSpec> {
                std::vec![$(<$children as ::bot_any_cal::Command>::spec()),*]
            }
        }

        impl ::bot_any_cal::Command for $name {
            const NAME: &'static str = "<root>";

            fn spec() -> ::bot_any_cal::CommandSpec {
                ::bot_any_cal::CommandSpec {
                    name: Self::NAME,
                    description: "",
                    options: ::std::vec::Vec::new(),
                    subcommands: ::std::vec![
                        $(
                            <$children as ::bot_any_cal::Command>::spec(),
                        )*
                    ],
                }
            }

            fn parse(preflights: &[::bot_any_cal::CommandPreflight]) -> Option<Self> {
                match preflights {
                    [::bot_any_cal::CommandPreflight::Select(name), rest @ ..] => {
                        match name.as_str() {
                            $(
                                <$children as ::bot_any_cal::Command>::NAME =>
                                    <$children as ::bot_any_cal::Command>::parse(rest).map($name::$children),
                            )*
                            _ => ::std::option::Option::None,
                        }
                    },
                    _ => ::std::option::Option::None,
                }
            }
        }
    }
}
