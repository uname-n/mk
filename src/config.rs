use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TaskSettings {
    pub commands: Vec<Vec<String>>,
    pub background_tasks: Option<Vec<Vec<String>>>,
}
