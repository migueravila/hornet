use crate::models::{Project, Todo};
use colored::*;
use std::collections::{HashMap, HashSet};

pub fn display_todos(todos: Vec<Todo>) {
    if todos.is_empty() {
        println!("\n  No todos for today!\n");
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

    println!("");

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
            "  {} {}",
            area.underline(),
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
            "  {} {}",
            project.underline(),
            format!("[0/{}]", total).dimmed()
        );

        for todo in project_todos.iter() {
            let id = todo_ids.get(&todo.name).unwrap_or(&0);
            println!("    ☐  {} {}", todo.name, format!("({})", id).dimmed());
        }
        println!("");
    }
}

pub fn display_projects(projects: Vec<Project>) {
    println!("");

    if projects.is_empty() {
        println!("  No projects found!");
        println!("");
        return;
    }

    let mut area_projects: HashMap<Option<String>, Vec<&Project>> = HashMap::new();

    for project in &projects {
        area_projects
            .entry(project.area_name.clone())
            .or_insert_with(Vec::new)
            .push(project);
    }

    if let Some(errands_projects) = area_projects.get(&None) {
        println!("  {}", "Errands".underline());
        for project in errands_projects {
            print!("    ⚪︎ {}", project.name.blue());

            if let Some(due) = &project.due_date {
                print!(" {}", due.dimmed());
            }
            println!();
        }

        println!("");
    }

    for (area_name, area_projects) in area_projects {
        if area_name.is_none() {
            continue;
        }

        let area = area_name.unwrap();
        let total = area_projects.len();

        println!(
            "  {} {}",
            area.underline(),
            format!("[0/{}]", total).dimmed()
        );

        for project in area_projects {
            print!("    ⚪︎ {}", project.name.blue());

            if let Some(due) = &project.due_date {
                print!(" {}", due.dimmed());
            }
            println!();
        }

        println!("");
    }

    println!("");
}

pub fn display_task_created(task_name: &str) {
    println!(
        "\n ✔  {}: {}\n",
        "Created task".green().bold(),
        task_name.bold()
    );
}

pub fn display_task_completed(task_name: &str) {
    println!(
        "\n ✓  {}: {}\n",
        "Completed task".green().bold(),
        task_name.bold()
    );
}
