mod api;
mod cli;
mod display;
mod models;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.value_of("title").unwrap();
        if let Some(project) = matches.value_of("project") {
            api::add_todo_to_project(title, project);
        } else {
            api::add_todo(title);
        }
    } else if matches.subcommand_matches("projects").is_some() {
        let projects = api::fetch_projects();
        display::display_projects(projects);
    } else if let Some(matches) = matches.subcommand_matches("done") {
        let id = matches.value_of("id").unwrap();
        match api::complete_todo(id) {
            Ok(message) => {
                let task_name = message.strip_prefix("Completed: ").unwrap_or(&message);
                display::display_task_completed(task_name);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        let todos = api::fetch_today_todos();
        display::display_todos(todos);
    }
}
