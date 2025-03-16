use super::common::execute_applescript;
use super::fetch::fetch_today_todos;

pub fn add_todo(title: &str) -> Result<String, String> {
    let script = format!(
        r#"
        tell application "Things3"
            set newToDo to make new to do with properties {{name:"{}"}} at beginning of list "Today"
            return "Todo added: " & name of newToDo
        end tell
        "#,
        title
    );

    execute_applescript(&script)
}

pub fn add_todo_to_project(title: &str, project_name: &str) -> Result<String, String> {
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

    let result = execute_applescript(&script)?;
    if result.starts_with("Error:") {
        return Err(result);
    }
    Ok(result)
}

pub fn complete_todo(todo_id: &str) -> Result<String, String> {
    let todos = fetch_today_todos()?;

    let index = match todo_id.parse::<usize>() {
        Ok(num) => {
            if num == 0 || num > todos.len() {
                return Err(format!(
                    "Invalid todo ID: {}. Valid range is 1-{}",
                    todo_id,
                    todos.len()
                ));
            }
            num - 1
        }
        Err(_) => return Err(format!("Invalid todo ID: {}. Must be a number.", todo_id)),
    };

    let todo = match todos.get(index) {
        Some(todo) => todo,
        None => return Err(format!("Todo with ID {} not found", todo_id)),
    };

    let script = format!(
        r#"
        tell application "Things3"
            set todayToDos to to dos of list "Today"
            repeat with i from 1 to count of todayToDos
                set toDo to item i of todayToDos
                if name of toDo is "{}" then
                    set status of toDo to completed
                    return "Completed: " & name of toDo
                end if
            end repeat
            return "Todo not found"
        end tell
        "#,
        todo.name
    );

    let result = execute_applescript(&script)?;
    if result == "Todo not found" {
        return Err("Todo not found in Things 3".to_string());
    }
    Ok(result)
}
