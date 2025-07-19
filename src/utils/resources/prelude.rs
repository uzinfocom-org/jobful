use serde::{Deserialize, Serialize};
use teloxide::types::{ChatId, MessageId, ThreadId};

pub static ADMINS: &[&str] = &["7598454972"];
pub static GROUPS: &str = include_str!("../../../groups.json");

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Chat {
    Ordinary {
        name: String,
        #[serde(rename = "chatId")]
        chat_id: ChatId,
    },
    Topic {
        name: String,
        #[serde(rename = "chatId")]
        chat_id: ChatId,
        topic: ThreadId,
    },
}

impl Chat {
    pub fn name(&self) -> &String {
        match self {
            Self::Ordinary { name, chat_id } => name,
            Self::Topic {
                name,
                chat_id,
                topic,
            } => name,
        }
    }

    pub fn chat_id(&self) -> &ChatId {
        match self {
            Self::Ordinary { name, chat_id } => chat_id,
            Self::Topic {
                name,
                chat_id,
                topic,
            } => chat_id,
        }
    }

    pub fn thread_id(&self) -> &ThreadId {
        match self {
            Self::Ordinary { name, chat_id } => &ThreadId(MessageId(0)),
            Self::Topic {
                name,
                chat_id,
                topic,
            } => topic,
        }
    }
}
