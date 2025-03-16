use super::common::execute_applescript;
use crate::models::Project;

pub fn fetch_projects() -> Result<Vec<Project>, String> {
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

    let output = execute_applescript(script)?;

    let mut projects = Vec::new();
    for line in output.lines() {
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

    Ok(projects)
}
