use crate::OSUtils;

// Mod commands
mod shell;

// Command trait
pub trait Command {
    fn matches(&self, text: &str) -> bool;

    fn exec(&self, ctx: Context);
}

// Context struct for commands
pub struct Context<'a> {
    pub command_raw: &'a str,
    pub command_args: Vec<&'a str>,
    pub utils: &'a mut OSUtils,
}

// A giant list of commands
pub const COMMANDS: [&dyn Command; 1] = [&shell::ShellCommand {}];
