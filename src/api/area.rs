use super::common::execute_applescript;
use crate::models::Area;

pub fn fetch_areas() -> Result<Vec<Area>, String> {
    let script = r#"
    tell application "Things3"
        set allAreas to areas
        set output to ""
        
        repeat with ar in allAreas
            -- Basic properties
            set areaName to name of ar
            
            -- Project count - wrap in try block to handle errors
            set projectCount to 0
            try
                set projectCount to count of (projects where area = ar)
            end try
            
            -- Todo count - wrap in try block to handle errors
            set todoCount to 0
            try
                set todoCount to count of (to dos where area = ar)
            end try
            
            -- Output a line with delimiter
            set output to output & areaName & "|~|" & projectCount & "|~|" & todoCount & "\n"
        end repeat
        
        return output
    end tell
    "#;

    let output = execute_applescript(script)?;

    let mut areas = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split("|~|").collect();
        if parts.len() >= 3 {
            let name = parts[0].to_string();
            let project_count = parts[1].parse::<usize>().unwrap_or(0);
            let todo_count = parts[2].parse::<usize>().unwrap_or(0);

            areas.push(Area {
                name,
                project_count,
                todo_count,
            });
        }
    }

    Ok(areas)
}
