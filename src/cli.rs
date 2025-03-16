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
        .subcommand(Command::new("areas").about("List all areas"))
        .subcommand(Command::new("tags").about("List all tags"))
        .subcommand(
            Command::new("done").about("Mark a todo as complete").arg(
                Arg::new("id")
                    .help("ID of the todo to complete")
                    .required(true),
            ),
        )
        .subcommand(Command::new("inbox").about("Show inbox todos"))
        .subcommand(Command::new("anytime").about("Show anytime todos"))
        .subcommand(Command::new("someday").about("Show someday todos"))
        .subcommand(Command::new("logbook").about("Show logbook todos"))
        .arg(
            Arg::new("tag")
                .help("Show todos with specific tag (use #tagname)")
                .required(false)
                .conflicts_with_all(&["project", "area"]),
        )
        .arg(
            Arg::new("project")
                .help("Show todos for specific project (use /projectname)")
                .required(false)
                .conflicts_with_all(&["tag", "area"]),
        )
        .arg(
            Arg::new("area")
                .help("Show todos for specific area (use @areaname)")
                .required(false)
                .conflicts_with_all(&["tag", "project"]),
        )
}
