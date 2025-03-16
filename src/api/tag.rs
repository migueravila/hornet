use super::common::execute_applescript;
use crate::models::Tag;

pub fn fetch_tags() -> Result<Vec<Tag>, String> {
    let script = r#"
    tell application "Things3"
        set allTags to tags
        set output to ""
        
        repeat with t in allTags
            -- Basic properties
            set tagName to name of t
            
            -- Todo count (this might be expensive operation for many tags)
            set taggedTodos to to dos where name of its tag is tagName
            set todoCount to count of taggedTodos
            
            -- Output a line with delimiter
            set output to output & tagName & "|~|" & todoCount & "\n"
        end repeat
        
        return output
    end tell
    "#;

    let output = execute_applescript(script)?;

    let mut tags = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split("|~|").collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let todo_count = parts[1].parse::<usize>().unwrap_or(0);

            tags.push(Tag { name, todo_count });
        }
    }

    Ok(tags)
}
