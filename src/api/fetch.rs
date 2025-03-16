use super::common::{execute_applescript, parse_todo_line, todo_extraction_script};
use crate::models::Todo;

pub fn fetch_today_todos() -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("built-in", Some("Today"));
    fetch_todos_with_script(&script)
}

pub fn fetch_inbox_todos() -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("built-in", Some("Inbox"));
    fetch_todos_with_script(&script)
}

pub fn fetch_anytime_todos() -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("built-in", Some("Anytime"));
    fetch_todos_with_script(&script)
}

pub fn fetch_someday_todos() -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("built-in", Some("Someday"));
    fetch_todos_with_script(&script)
}

pub fn fetch_logbook_todos() -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("built-in", Some("Logbook"));
    fetch_todos_with_script(&script)
}

pub fn fetch_area_todos(area_name: &str) -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("area", Some(area_name));
    fetch_todos_with_script(&script)
}

pub fn fetch_project_todos(project_name: &str) -> Result<Vec<Todo>, String> {
    let script = todo_extraction_script("project", Some(project_name));
    fetch_todos_with_script(&script)
}

fn fetch_todos_with_script(script: &str) -> Result<Vec<Todo>, String> {
    let output = execute_applescript(script)?;

    let mut todos = Vec::new();
    for line in output.lines() {
        if let Some(todo) = parse_todo_line(line) {
            todos.push(todo);
        }
    }

    Ok(todos)
}

pub fn fetch_tag_todos(tag_name: &str) -> Result<Vec<Todo>, String> {
    let script = format!(
        r#"
        tell application "Things3"
            try
                -- First check if the tag exists
                set theTag to missing value
                set allTags to tags
                repeat with t in allTags
                    if name of t is "{}" then
                        set theTag to t
                        exit repeat
                    end if
                end repeat
                
                if theTag is missing value then
                    return "ERROR: Tag '{}' not found"
                end if
                
                -- Get todos with this tag - CORRECT WAY TO ACCESS TAG TODOS
                set taggedToDos to to dos of theTag
                set output to ""
                
                repeat with toDo in taggedToDos
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
            on error errMsg
                return "ERROR: " & errMsg
            end try
        end tell
        "#,
        tag_name, tag_name
    );

    let output = execute_applescript(&script)?;

    // Check if there was an error in the AppleScript execution
    if output.starts_with("ERROR:") {
        return Err(output
            .strip_prefix("ERROR: ")
            .unwrap_or(&output)
            .to_string());
    }

    let mut todos = Vec::new();
    for line in output.lines() {
        if let Some(todo) = parse_todo_line(line) {
            todos.push(todo);
        }
    }

    Ok(todos)
}
