use crate::{Flag, Intent, CommandSummary, build_command_positions,
    command_at_position, build_supcommand_summaries, build_subcommand_summaries,
    summarizebuild_flag_summaries};

/// Command structure which represents command-line task.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    name: String,
    description: Option<String>,
    flags: Vec<Flag>,
    commands: Vec<Command>,
    resolver: Option<fn(Intent) -> Option<usize>>,
}

/// Command structure implementation.
impl Command {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns flags.
    pub fn flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    /// Returns commands.
    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    /// Builds summary for this command.
    pub fn summarize(&self) -> CommandSummary {
        CommandSummary::new(
            self.name.clone().as_str(),
            self.description.clone(),
        )
    }
}

/// Command structure implementation.
impl Command {

    /// Returns new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            flags: Vec::new(),
            commands: Vec::new(),
            resolver: None,
        }
    }

    /// Sets description.
    pub fn with_description(mut self, val: &str) -> Self {
        self.description = Some(val.to_string());
        self
    }
    
    /// Sets resolver function.
    pub fn with_resolver(mut self, r: fn(Intent) -> Option<usize>) -> Self {
        self.resolver = Some(r);
        self
    }

    /// Adds flag.
    pub fn with_flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Adds subcommand.
    pub fn with_subcommand(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    /// Executes as a command-line application.
    pub fn perform(self, args: Vec<String>) -> Option<usize> {
        let command_positions = build_command_positions(&self, &args);
        let command = command_at_position(&self, &command_positions);

        let command_summary = command.summarize();
        let supcommand_summary = build_supcommand_summaries(&self, &command_positions);
        let subcommand_summary = build_subcommand_summaries(&command);
        let flag_snapeshots = summarizebuild_flag_summaries(&command);

        let intent = Intent::new(
            args,
            command_summary,
            supcommand_summary,
            subcommand_summary,
            flag_snapeshots,
        );

        match command.resolver {
            Some(resolver) => resolver(intent),
            None => panic!("resolver not set"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_command() {
        fn resolver0(_: Intent) -> Option<usize> { Some(0) };
        let app = Command::new("a")
            .with_resolver(resolver0);
        assert_eq!(app.perform(vec![]), Some(0));
    }

    #[test]
    fn performs_subcommand() {
        fn resolver0(_: Intent) -> Option<usize> { Some(0) };
        fn resolver1(_: Intent) -> Option<usize> { Some(1) };
        let app = Command::new("a")
            .with_subcommand(
                Command::new("b")
                    .with_resolver(resolver1)
            )
            .with_resolver(resolver0);
        assert_eq!(app.perform(vec!["b".to_string()]), Some(1));
    }

    #[test]
    fn builds_summary() {
        let app = Command::new("name")
            .with_description("description");
        assert_eq!(app.summarize(), CommandSummary::new(
            "name",
            Some("description".to_string()),
        ));
    }
}