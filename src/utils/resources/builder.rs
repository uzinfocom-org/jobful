use super::Resources;
use super::{prelude, prelude::*};
use super::{APP_USER_AGENT, BASE};
use crate::{JobfulErrors, Result};
use reqwest::Client;
use std::time::SystemTime;
use teloxide::types::UserId;

#[derive(Clone)]
pub struct ResourcesBuilder {
    data: Option<Jobs>,
    admins: Option<Vec<UserId>>,
    client: Option<Client>,
    timestamp: Option<SystemTime>,
}

impl Default for ResourcesBuilder {
    fn default() -> Self {
        Self {
            timestamp: None,
            data: None,
            admins: Some(
                prelude::ADMINS
                    .iter()
                    .map(|a| UserId(a.parse().unwrap()))
                    .collect(),
            ),
            client: None,
        }
    }
}

impl ResourcesBuilder {
    pub fn httpclient(self) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        Ok(Self {
            data: self.data,
            admins: self.admins,
            client: Some(client),
            timestamp: self.timestamp,
        })
    }

    pub async fn initiate(self) -> Result<Self> {
        let client = match self.client {
            Some(c) => c,
            None => return Err(JobfulErrors::NoHTTPClient),
        };

        let data = client
            .get(format!(
                "{BASE}/api/v1/company/vacancies/?page=1&page_size=100"
            ))
            .send()
            .await
            .map_err(JobfulErrors::Reqwest)?
            .json::<Jobsonse>()
            .await
            .map_err(JobfulErrors::Reqwest)?
            .results;

        Ok(Self {
            data: Some(data),
            admins: self.admins,
            client: Some(client),
            timestamp: Some(SystemTime::now()),
        })
    }

    pub fn build(self) -> Result<Resources> {
        let data = match self.data {
            Some(d) => d,
            None => return Err(JobfulErrors::MissingDependency),
        };

        let client = match self.client {
            Some(c) => c,
            None => return Err(JobfulErrors::MissingDependency),
        };

        let admins = match self.admins {
            Some(c) => c,
            None => return Err(JobfulErrors::MissingDependency),
        };

        let timestamp = match self.timestamp {
            Some(t) => t,
            None => return Err(JobfulErrors::MissingDependency),
        };

        Ok(Resources {
            data,
            admins,
            client,
            timestamp,
        })
    }
}
