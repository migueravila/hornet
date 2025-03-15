#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub notes: String,
    pub project_name: Option<String>,
    pub area_name: Option<String>,
    pub due_date: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub area_name: Option<String>,
    pub due_date: Option<String>,
    pub task_count: usize,
}
