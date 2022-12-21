use std::{
    io::Write,
    process::{ChildStdin, Command, Stdio},
};

pub struct OSUtil {
    shell: ChildStdin,
}

impl super::OSUtilMethods for OSUtil {
    fn new() -> Self {
        Self {
            shell: Self::shell(),
        }
    }

    fn shell() -> ChildStdin {
        // Spawn a new shell via the command prompt
        Command::new("C:\\Windows\\System32\\cmd.exe")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("Error spawning shell!")
            // Take the shell's StdIn, so we can write to it later
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
