use crate::models::{Project, Todo};
use std::process::Command;

pub fn fetch_today_todos() -> Vec<Todo> {
    let script = r#"
    tell application "Things3"
        set todayToDos to to dos of list "Today"
        set output to ""
        
        repeat with toDo in todayToDos
            -- Basic properties
            set todoName to name of toDo
            set todoNotes to notes of toDo
            
            -- Project info
            set projectName to "NULL"
            try
                set pr to project of toDo
                if pr is not missing value then
                    set projectName to name of pr
                end if
            end try
            
            -- Area info
            set areaName to "NULL"
            try
                if project of toDo is missing value then
                    try
                        set ar to area of toDo
                        if ar is not missing value then
                            set areaName to name of ar
                        end if
                    end try
                else
                    try
                        set ar to area of project of toDo
                        if ar is not missing value then
                            set areaName to name of ar
                        end if
                    end try
                end if
            end try
            
            -- Due date
            set dueDateStr to "NULL"
            try
                if due date of toDo is not missing value then
                    set dueDateStr to (due date of toDo as string)
                end if
            end try
            
            -- Tags
            set tagStr to ""
            repeat with t in tags of toDo
                set tagStr to tagStr & name of t & ";"
            end repeat
            if tagStr is "" then set tagStr to "NULL"
            
            -- Output a line with delimiter
            set output to output & todoName & "|~|" & todoNotes & "|~|" & projectName & "|~|" & areaName & "|~|" & dueDateStr & "|~|" & tagStr & "\n"
        end repeat
        
        return output
    end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    let mut todos = Vec::new();

    if output.status.success() {
        let raw_output = String::from_utf8_lossy(&output.stdout);

        for line in raw_output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split("|~|").collect();
            if parts.len() >= 6 {
                let name = parts[0].to_string();
                let notes = parts[1].to_string();
                let project_name = if parts[2] == "NULL" {
                    None
                } else {
                    Some(parts[2].to_string())
                };
                let area_name = if parts[3] == "NULL" {
                    None
                } else {
                    Some(parts[3].to_string())
                };
                let due_date = if parts[4] == "NULL" {
                    None
                } else {
                    Some(parts[4].to_string())
                };

                let tags = if parts[5] == "NULL" {
                    Vec::new()
                } else {
                    parts[5]
                        .split(';')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect()
                };

                todos.push(Todo {
                    name,
                    notes,
                    project_name,
                    area_name,
                    due_date,
                    tags,
                });
            }
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    todos
}

pub fn fetch_projects() -> Vec<Project> {
    let script = r#"
    tell application "Things3"
        set allProjects to projects
        set output to ""
        
        repeat with pr in allProjects
            -- Basic properties
            set projectName to name of pr
            
            -- Area info
            set areaName to "NULL"
            try
                set ar to area of pr
                if ar is not missing value then
                    set areaName to name of ar
                end if
            end try
            
            -- Due date
            set dueDateStr to "NULL"
            try
                if due date of pr is not missing value then
                    set dueDateStr to (due date of pr as string)
                end if
            end try
            
            -- Task count
            set taskCount to count of to dos of pr
            
            -- Output a line with delimiter
            set output to output & projectName & "|~|" & areaName & "|~|" & dueDateStr & "|~|" & taskCount & "\n"
        end repeat
        
        return output
    end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    let mut projects = Vec::new();

    if output.status.success() {
        let raw_output = String::from_utf8_lossy(&output.stdout);

        for line in raw_output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split("|~|").collect();
            if parts.len() >= 4 {
                let name = parts[0].to_string();
                let area_name = if parts[1] == "NULL" {
                    None
                } else {
                    Some(parts[1].to_string())
                };
                let due_date = if parts[2] == "NULL" {
                    None
                } else {
                    Some(parts[2].to_string())
                };
                let task_count = parts[3].parse::<usize>().unwrap_or(0);

                projects.push(Project {
                    name,
                    area_name,
                    due_date,
                    task_count,
                });
            }
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    projects
}

pub fn add_todo(title: &str) {
    let script = format!(
        r#"
        tell application "Things3"
            set newToDo to make new to do with properties {{name:"{}"}} at beginning of list "Today"
            return "Todo added: " & name of newToDo
        end tell
        "#,
        title
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn add_todo_to_project(title: &str, project_name: &str) {
    let script = format!(
        r#"
        tell application "Things3"
            try
                set pr to project named "{}"
                set newToDo to make new to do with properties {{name:"{}"}} at beginning of project "{}"
                move newToDo to list "Today"
                return "Todo added to project " & name of pr & ": " & name of newToDo
            on error
                return "Error: Project '{}' not found"
            end try
        end tell
        "#,
        project_name, title, project_name, project_name
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
