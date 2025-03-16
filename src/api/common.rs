use std::process::Command;

pub(crate) fn execute_applescript(script: &str) -> Result<String, String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub(crate) fn parse_todo_line(line: &str) -> Option<crate::models::Todo> {
    if line.trim().is_empty() {
        return None;
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

        Some(crate::models::Todo {
            name,
            notes,
            project_name,
            area_name,
            due_date,
            tags,
        })
    } else {
        None
    }
}

pub(crate) fn todo_extraction_script(list_type: &str, list_name: Option<&str>) -> String {
    let list_identifier = match list_type {
        "built-in" => format!("list \"{}\"", list_name.unwrap_or("Today")),
        "area" => format!("area named \"{}\"", list_name.unwrap_or("")),
        "project" => format!("project named \"{}\"", list_name.unwrap_or("")),
        "tag" => format!(
            "to dos where name of its tag is \"{}\"",
            list_name.unwrap_or("")
        ),
        _ => format!("list \"Today\""),
    };

    format!(
        r#"
    tell application "Things3"
        set theToDos to to dos of {}
        set output to ""
        
        repeat with toDo in theToDos
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
    "#,
        list_identifier
    )
}
