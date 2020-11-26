use crate::utils::{exec, pckg};
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Spawn")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["spawn".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let (print_output, start_index) = if !arguments.is_empty() && arguments[0] == "--silent" {
            (false, 1)
        } else {
            (true, 0)
        };

        match exec::spawn(&arguments, print_output, false, start_index) {
            Ok(child) => {
                let pid = child.id();

                CommandResult::Continue(Some(pid.to_string()))
            }
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
