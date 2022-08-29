use kal::command_group;

pub use unknown::handle_unknown;
pub use version::Version;
pub use works_left::WorksLeft;

mod unknown;
mod version;
mod works_left;

command_group! {
    RootCommand {
        WorksLeft,
        Version,
    }
}
