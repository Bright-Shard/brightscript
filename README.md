# BrightScript
An open-source scripting language intended for cross-platform HID attacks. It's currently WIP, but will have universal commands for common tasks (downloading files, running executables, etc) that are cross-platform. That's right - no more OS-specific payloads!

And, yes, BrightScript is unironically abbreviated BS. You're welcome.




# Programming
Every line of code must end with `;`. Beyond that, BrightScript has two forms: 'Long Form' that's easier to read, and 'Short Form' that has smaller payloads. You can use Long Form and Short Form interchangeably in the same script; however, this isn't recommended, as it makes your script harder to read.

In the future, I hope to make a 'compressor' program that will convert Long Form BrightScript into Short Form BrightScript. This way, open-source scripts can remain in long form, and be easily converted into Short Form for smaller payload sizes.


## Long Form
The command name must always be the first word in a line of code. Arguments are divided by whitespace/spaces. Each line of code should look like this:

`<command_name> <arg1> <arg2> <arg3>...;`


## Short Form
The command symbol must always be the first character in a line of code. Arguments **immediately** follow the symbol, and are split by whitespace (spaces). Each line of code should look like this:

`<command_symbol><arg1> <arg2> <arg3>...;`

Notice that there **is not a space** between the symbol and first argument in Short Form mode. Also, some commands may follow slightly different syntax in Short Form mode.


## Remember your semicolons!
**Remember your semicolons!** The BrightScript parser won't be able to tell the difference between two lines of code without them, and doesn't have an incredible compiler like Rust that will catch an issue for you. You'll just have very, very weird results.




# Commands
- Shell:
  - Description: Run a command directly in the terminal.
  - Short Form/Command Symbol: `$`
  - Long Form/Command Name: `shell`
  - Notes: **This command isn't cross-platform!** Different operating systems use different shells. Although macOS and Linux have (mostly) the same terminal commands, Windows is stupid and has a different set of commands for almost everything. This command is mostly intended as a back-up in case BrightScript doesn't yet have a feature you need.

