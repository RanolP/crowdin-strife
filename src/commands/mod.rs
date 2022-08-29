use kal::command_group;

pub use test::TestCommand;
pub use unknown::handle_unknown;
pub use version::Version;
pub use works_left::WorksLeft;

mod test;
mod unknown;
mod version;
mod works_left;

command_group! {
    RootCommand {
        TestCommand,
        WorksLeft,
        Version,
    }
}
