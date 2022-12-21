use crate::OSUtilMethods;

use super::Command;

pub struct ShellCommand {}
impl Command for ShellCommand {
    fn matches(&self, text: &str) -> bool {
        text.starts_with('$')
    }
    fn exec(&self, ctx: super::Context) {
        let mut command_chars = ctx.command_raw.chars();
        command_chars.next();
        let command = command_chars.as_str().to_string();

        ctx.utils.exec(command).unwrap();
    }
}
