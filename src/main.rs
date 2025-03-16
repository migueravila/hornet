mod api;
mod cli;
mod display;
mod models;

fn handle_add_command(title: &str, project: Option<&str>) {
    let result = if let Some(project_name) = project {
        api::add_todo_to_project(title, project_name)
    } else {
        api::add_todo(title)
    };

    match result {
        Ok(message) => {
            let task_name = message
                .strip_prefix("Todo added: ")
                .or_else(|| message.split(": ").last())
                .unwrap_or(&message);
            display::display_task_created(task_name);
        }
        Err(err) => display::display_error(&err),
    }
}

fn handle_done_command(id: &str) {
    match api::complete_todo(id) {
        Ok(message) => {
            let task_name = message.strip_prefix("Completed: ").unwrap_or(&message);
            display::display_task_completed(task_name);
        }
        Err(err) => display::display_error(&err),
    }
}

fn handle_projects_command() {
    match api::fetch_projects() {
        Ok(projects) => display::display_projects(projects),
        Err(err) => display::display_error(&err),
    }
}

fn handle_today_command() {
    match api::fetch_today_todos() {
        Ok(todos) => display::display_todos(todos),
        Err(err) => display::display_error(&err),
    }
}

fn handle_inbox_command() {
    match api::fetch_inbox_todos() {
        Ok(todos) => display::display_todos_with_header(todos, "Inbox"),
        Err(err) => display::display_error(&err),
    }
}

fn handle_anytime_command() {
    match api::fetch_anytime_todos() {
        Ok(todos) => display::display_todos_with_header(todos, "Anytime"),
        Err(err) => display::display_error(&err),
    }
}

fn handle_someday_command() {
    match api::fetch_someday_todos() {
        Ok(todos) => display::display_todos_with_header(todos, "Someday"),
        Err(err) => display::display_error(&err),
    }
}

fn handle_logbook_command() {
    match api::fetch_logbook_todos() {
        Ok(todos) => display::display_todos_with_header(todos, "Logbook"),
        Err(err) => display::display_error(&err),
    }
}

fn handle_tag_command(tag_name: &str) {
    let tag = tag_name.trim_start_matches('#');
    match api::fetch_tag_todos(tag) {
        Ok(todos) => display::display_todos_with_header(todos, &format!("Tag: #{}", tag)),
        Err(err) => display::display_error(&err),
    }
}

fn handle_project_command(project_name: &str) {
    let project = project_name.trim_start_matches('/');
    match api::fetch_project_todos(project) {
        Ok(todos) => display::display_todos_with_header(todos, &format!("Project: {}", project)),
        Err(err) => display::display_error(&err),
    }
}

fn handle_area_command(area_name: &str) {
    let area = area_name.trim_start_matches('@');
    match api::fetch_area_todos(area) {
        Ok(todos) => display::display_todos_with_header(todos, &format!("Area: {}", area)),
        Err(err) => display::display_error(&err),
    }
}

fn handle_areas_command() {
    match api::fetch_areas() {
        Ok(areas) => display::display_areas(areas),
        Err(err) => display::display_error(&err),
    }
}

fn handle_tags_command() {
    match api::fetch_tags() {
        Ok(tags) => display::display_tags(tags),
        Err(err) => display::display_error(&err),
    }
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let first_arg = &args[1];

        if first_arg.starts_with('#') {
            handle_tag_command(first_arg);
            return;
        } else if first_arg.starts_with('/') {
            handle_project_command(first_arg);
            return;
        } else if first_arg.starts_with('@') {
            handle_area_command(first_arg);
            return;
        }
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.value_of("title").unwrap();
        let project = matches.value_of("project");
        handle_add_command(title, project);
    } else if matches.subcommand_matches("projects").is_some() {
        handle_projects_command();
    } else if matches.subcommand_matches("areas").is_some() {
        handle_areas_command();
    } else if matches.subcommand_matches("tags").is_some() {
        handle_tags_command();
    } else if let Some(matches) = matches.subcommand_matches("done") {
        let id = matches.value_of("id").unwrap();
        handle_done_command(id);
    } else if matches.subcommand_matches("inbox").is_some() {
        handle_inbox_command();
    } else if matches.subcommand_matches("anytime").is_some() {
        handle_anytime_command();
    } else if matches.subcommand_matches("someday").is_some() {
        handle_someday_command();
    } else if matches.subcommand_matches("logbook").is_some() {
        handle_logbook_command();
    } else {
        handle_today_command();
    }
}
