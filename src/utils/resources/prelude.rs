use serde::{Deserialize, Serialize};

pub static ADMINS: &[&str] = &["7598454972"];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Specialization {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub experience: String,
    pub views: u32,
    pub date: String,
    pub location: String,
    pub salary: String,
    pub employ_type: String,
    pub specialization: Specialization,
}

pub type Jobs = Vec<Job>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Jobsonse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Jobs,
}
