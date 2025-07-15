#![allow(unused_assignments)]
pub mod builder;
pub mod prelude;

use crate::{JobfulErrors, Result};
use builder::ResourcesBuilder;
use prelude::*;
use reqwest::Client;
use rust_fuzzy_search::fuzzy_search_sorted;
use teloxide::types::UserId;

const BASE: &str = "https://uzinfocom.uz";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Clone, Debug)]
pub struct Resources {
    data: Jobs,
    admins: Vec<UserId>,
    client: Client,
}

impl Resources {
    pub fn builder() -> ResourcesBuilder {
        ResourcesBuilder::default()
    }

    fn get_titles(&self) -> Vec<&str> {
        self.data.iter().map(|d| d.title.as_ref()).collect()
    }

    pub fn search<T>(&self, param: T, amount: usize) -> Jobs
    where
        T: AsRef<str>,
    {
        fuzzy_search_sorted(param.as_ref(), self.get_titles().as_ref())
            .into_iter()
            .map(|j| {
                self.data
                    .iter()
                    .find(|d| d.title == j.0)
                    .unwrap()
                    .to_owned()
            })
            .take(amount)
            .collect()
    }

    pub fn is_admin(&self, user: &UserId) -> bool {
        self.admins.contains(user)
    }

    pub async fn update(mut self) -> Result<()> {
        let data: Jobsonse = match match self
            .client
            .get(format!(
                "{BASE}/api/v1/company/vacancies/?format=json&page=1&page_size=1000"
            ))
            .send()
            .await
        {
            Ok(d) => d.json::<Jobsonse>().await,
            Err(e) => return Err(JobfulErrors::Reqwest(e)),
        } {
            Ok(sd) => sd,
            Err(e) => return Err(JobfulErrors::Reqwest(e)),
        };

        self = Self {
            client: self.client,
            admins: self.admins,
            data: data.results,
        };

        Ok(())
    }
}
