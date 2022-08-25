#[macro_export]
macro_rules! command_group {
    ($name:ident { $($children:ident,)* }) => {
        pub enum $name {
            $($children($children)),*
        }

        impl ::bot_any_cal::Command for $name {
            const NAME: &'static str = "<root>";
            
            fn spec() -> ::bot_any_cal::CommandSpec {
                ::bot_any_cal::CommandSpec {
                    name: Self::NAME,
                    description: ::std::option::Option::None,
                    options: ::std::vec::Vec::new(),
                    subcommands: ::std::vec![
                        $(
                            $children::spec(),
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
