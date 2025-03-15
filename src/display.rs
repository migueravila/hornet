use crate::models::{Project, Todo};

pub fn display_todos(todos: Vec<Todo>) {
    println!("Today's Todos:");

    if todos.is_empty() {
        println!("  No todos for today!");
        return;
    }

    for todo in todos {
        println!("• {}", todo.name);

        if let Some(project) = &todo.project_name {
            println!("  Project: {}", project);
        }

        if let Some(area) = &todo.area_name {
            println!("  Area: {}", area);
        }

        if !todo.notes.is_empty() {
            println!("  Notes: {}", todo.notes);
        }

        if let Some(due) = &todo.due_date {
            println!("  Due: {}", due);
        }

        if !todo.tags.is_empty() {
            println!("  Tags: {}", todo.tags.join(", "));
        }

        println!(); // Empty line between todos
    }
}

pub fn display_projects(projects: Vec<Project>) {
    println!("Projects:");

    if projects.is_empty() {
        println!("  No projects found!");
        return;
    }

    for project in projects {
        println!("• {}", project.name);

        if let Some(area) = &project.area_name {
            println!("  Area: {}", area);
        }

        if let Some(due) = &project.due_date {
            println!("  Due: {}", due);
        }

        println!("  Tasks: {}", project.task_count);
        println!(); // Empty line between projects
    }
}
