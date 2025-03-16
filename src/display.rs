use crate::models::{Area, Project, Tag, Todo};
use colored::*;
use std::collections::{HashMap, HashSet};

fn print_border_line() {
    println!("{}", "─".repeat(60).bright_black());
}

pub fn display_todos(todos: Vec<Todo>) {
    print_border_line();

    if todos.is_empty() {
        println!("  No todos for today!");
        print_border_line();
        return;
    }

    let mut id_counter = 1;
    let mut todo_ids: HashMap<String, usize> = HashMap::new();

    for todo in &todos {
        todo_ids.insert(todo.name.clone(), id_counter);
        id_counter += 1;
    }

    let mut standalone_todos = Vec::new();
    let mut area_todos: HashMap<String, Vec<&Todo>> = HashMap::new();
    let mut project_todos: HashMap<String, Vec<&Todo>> = HashMap::new();
    let mut covered_todos = HashSet::new();

    for todo in &todos {
        match (&todo.area_name, &todo.project_name) {
            (Some(area), None) => {
                area_todos
                    .entry(area.clone())
                    .or_insert_with(Vec::new)
                    .push(todo);
                covered_todos.insert(todo.name.clone());
            }
            (_, Some(project)) => {
                project_todos
                    .entry(project.clone())
                    .or_insert_with(Vec::new)
                    .push(todo);
                covered_todos.insert(todo.name.clone());
            }
            _ => {}
        }
    }

    for todo in &todos {
        if !covered_todos.contains(&todo.name) {
            standalone_todos.push(todo);
        }
    }

    if !standalone_todos.is_empty() {
        println!("  {}", "Errands".underline());

        for todo in standalone_todos.iter() {
            let id = todo_ids.get(&todo.name).unwrap_or(&0);
            println!("    ☐  {} {}", todo.name, format!("({})", id).dimmed());
        }
        println!("");
    }

    for (area, area_todos) in area_todos {
        let total = area_todos.len();
        println!(
            "  @ {} {}",
            area.cyan().bold(),
            format!("[0/{}]", total).dimmed()
        );

        for todo in area_todos.iter() {
            let id = todo_ids.get(&todo.name).unwrap_or(&0);
            println!("    ☐  {} {}", todo.name, format!("({})", id).dimmed());
        }
        println!("");
    }

    for (project, project_todos) in project_todos {
        let total = project_todos.len();
        println!(
            "  / {} {}",
            project.blue().bold(),
            format!("[0/{}]", total).dimmed()
        );

        for todo in project_todos.iter() {
            let id = todo_ids.get(&todo.name).unwrap_or(&0);
            println!("    ☐  {} {}", todo.name, format!("({})", id).dimmed());
        }
        println!("");
    }

    print_border_line();
}

pub fn display_task_created(task_name: &str) {
    print_border_line();
    println!(
        " ✔  {}: {}",
        "Created task".green().bold(),
        task_name.bold()
    );
    print_border_line();
}

pub fn display_task_completed(task_name: &str) {
    print_border_line();
    println!(
        " ✓  {}: {}",
        "Completed task".green().bold(),
        task_name.bold()
    );
    print_border_line();
}

pub fn display_error(message: &str) {
    print_border_line();
    println!(" ✗  {}: {}", "Error".red().bold(), message);
    print_border_line();
}

pub fn display_todos_with_header(todos: Vec<Todo>, header: &str) {
    print_border_line();

    if todos.is_empty() {
        println!("  No todos in {}!", header);
        print_border_line();
        return;
    }

    println!("  {}", header.bold().underline());
    display_todos(todos);
}

pub fn display_areas(areas: Vec<Area>) {
    print_border_line();

    if areas.is_empty() {
        println!("  No areas found!");
        print_border_line();
        return;
    }

    println!("  {}", "Areas".bold().underline());
    println!("");

    for area in areas {
        println!(
            "  @{} {} {}",
            area.name.cyan().bold(),
            format!("Projects: {}", area.project_count).dimmed(),
            format!("Todos: {}", area.todo_count).dimmed()
        );
    }

    print_border_line();
}

pub fn display_tags(tags: Vec<Tag>) {
    print_border_line();

    if tags.is_empty() {
        println!("  No tags found!");
        print_border_line();
        return;
    }

    println!("  {}", "Tags".bold().underline());
    println!("");

    for tag in tags {
        println!(
            "  #{} {}",
            tag.name.yellow().bold(),
            format!("({})", tag.todo_count).dimmed()
        );
    }

    print_border_line();
}

pub fn display_projects(projects: Vec<Project>) {
    print_border_line();

    if projects.is_empty() {
        println!("  No projects found!");
        print_border_line();
        return;
    }

    println!("  {}", "Projects".bold().underline());
    println!("");

    for project in &projects {
        print!("  /{}", project.name.blue().bold());
        print!(" {}", format!("Todo: {}", project.task_count).dimmed());

        if let Some(area_name) = &project.area_name {
            print!(" {} {}", "Area:".dimmed(), area_name.cyan());
        }

        if let Some(due) = &project.due_date {
            print!(" {} {}", "Due:".dimmed(), due.dimmed());
        }

        println!();
    }

    print_border_line();
}
