use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TaskSettings {
    pub command: Vec<Vec<String>>,
    pub background_tasks: Option<Vec<Vec<String>>>,
}
