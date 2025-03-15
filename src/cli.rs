use clap::{Arg, Command};

pub fn build_cli() -> Command<'static> {
    Command::new("hornet")
        .version("0.1.0")
        .author("Miguel Avila")
        .about("Things 3 for powerusers")
        .subcommand(
            Command::new("add")
                .about("Add a new todo to Things3")
                .arg(Arg::new("title").help("Todo title").required(true))
                .arg(
                    Arg::new("project")
                        .short('p')
                        .long("project")
                        .help("Project name to assign this todo to")
                        .takes_value(true),
                ),
        )
        .subcommand(Command::new("projects").about("List all projects"))
        .subcommand(
            Command::new("done").about("Mark a todo as complete").arg(
                Arg::new("id")
                    .help("ID of the todo to complete")
                    .required(true),
            ),
        )
}
