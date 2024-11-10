use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct TaskSettings {
    pub commands: Vec<Vec<String>>,
    pub background_tasks: Option<Vec<Vec<String>>>,
}

impl<'de> Deserialize<'de> for TaskSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawTaskSettings {
            commands: VecOrVecVecString,
            background_tasks: Option<MixedVecOrVecVecString>,
        }

        let raw = RawTaskSettings::deserialize(deserializer)?;

        Ok(TaskSettings {
            commands: raw.commands.into_vec_vec(),
            background_tasks: raw.background_tasks.map(|bg| bg.into_vec_vec()),
        })
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum VecOrVecVecString {
    Vec(Vec<String>),
    VecVec(Vec<Vec<String>>),
}

impl VecOrVecVecString {
    fn into_vec_vec(self) -> Vec<Vec<String>> {
        match self {
            VecOrVecVecString::Vec(v) => v.into_iter().map(|s| vec![s]).collect(),
            VecOrVecVecString::VecVec(vv) => vv,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MixedVecOrVecVecString {
    Single(VecOrVecVecString),
    Mixed(Vec<MixedStringOrVec>),
}

impl MixedVecOrVecVecString {
    fn into_vec_vec(self) -> Vec<Vec<String>> {
        match self {
            MixedVecOrVecVecString::Single(single) => single.into_vec_vec(),
            MixedVecOrVecVecString::Mixed(mixed) => mixed.into_iter().map(|item| item.into_vec()).collect(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MixedStringOrVec {
    Single(String),
    Vec(Vec<String>),
}

impl MixedStringOrVec {
    fn into_vec(self) -> Vec<String> {
        match self {
            MixedStringOrVec::Single(s) => vec![s],
            MixedStringOrVec::Vec(v) => v,
        }
    }
}