use std::{
    io::Write,
    process::{ChildStdin, Command, Stdio},
};

pub struct OSUtils {
    shell: ChildStdin,
}

impl super::OSUtilMethods for OSUtils {
    fn new() -> Self {
        Self {
            shell: Self::shell(),
        }
    }

    fn shell() -> ChildStdin {
        // Spawn a new zsh shell (zsh is the default shell on macOS)
        Command::new("/bin/zsh")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Error spawning shell!")
            // Take StdIn
            .stdin
            .take()
            .expect("Error getting shell's StdIn!")
    }

    fn exec(&mut self, mut command: String) -> std::io::Result<()> {
        command += "\n";
        self.shell.write_all(command.as_bytes())?;
        Ok(())
    }
}
