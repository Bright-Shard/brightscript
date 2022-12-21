use crate::{
    commands::{Context, COMMANDS},
    OSUtilMethods, OSUtils,
};

pub struct Parser {
    utils: OSUtils,
}
impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
impl Parser {
    pub fn new() -> Self {
        Self {
            utils: OSUtils::new(),
        }
    }
    pub fn with_util(utils: OSUtils) -> Self {
        Self { utils }
    }
    pub fn parse(&mut self, script: String) {
        let commands = script.split(';');

        for command_raw in commands {
            if !command_raw.is_empty() {
                // Split command and arguments
                let mut command_and_args = <&str>::clone(&command_raw).split_whitespace();
                // Just the command name
                let command_name = command_and_args.next().unwrap();
                // Just the command arguments
                let command_args = command_and_args.collect();

                for bs_command in COMMANDS {
                    if bs_command.matches(command_name) {
                        bs_command.exec(Context {
                            command_raw,
                            command_args,
                            utils: &mut self.utils,
                        });
                        break;
                    }
                }
            }
        }
    }
}
