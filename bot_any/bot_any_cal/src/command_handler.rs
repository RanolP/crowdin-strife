use crate::Command;

pub trait CommandHandler<T: Command>: Sized {
    fn visit_subcommand(self, name: &str) -> Option<Self>;

    fn visit_argument_str(self, key: String, value: String) -> Self;
    
    fn visit_argument_i64(self, key: String, value: i64) -> Self;
    
    fn visit_argument_f64(self, key: String, value: f64) -> Self;

    fn parse(self) -> T;
}
