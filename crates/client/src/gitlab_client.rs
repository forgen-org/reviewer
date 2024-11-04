use chrono::{DateTime, TimeZone, Utc};
use futures::future::join_all;
use gloo_storage::{LocalStorage, Storage};
use log::*;
use serde::{Deserialize, Serialize};

use crate::{change_request::ChangeRequest, parsed_note::ParsedNote};

pub struct GitlabClient {
    access_token: String,
    domain: String,
    project: String,
}

impl GitlabClient {
    pub fn new(access_token: String, project: String) -> Self {
        let domain = "gitlab.com".to_string();
        let project = urlencoding::encode(&project).to_string();
        Self {
            access_token,
            domain,
            project,
        }
    }

    pub fn get(&self, endpoint: &str) -> reqwest::RequestBuilder {
        let url = format!(
            "https://{}/api/v4/projects/{}/{}",
            self.domain, self.project, endpoint
        );
        let client = reqwest::Client::new();
        client
            .get(url)
            .header("accept", "application/json")
            .header("private-token", self.access_token.clone())
            .query(&[("per_page", "100")])
    }

    pub async fn fetch(&self) -> Vec<ChangeRequest> {
        let cache: Option<Cache> = LocalStorage::get("change_requests").ok();

        if let Some(cache) = &cache {
            if Utc::now() < cache.from + chrono::Duration::minutes(5) {
                return cache.change_requests.clone();
            }
        }

        let from = match &cache {
            Some(cache) => cache.from,
            None => Utc.with_ymd_and_hms(2024, 10, 28, 12, 0, 0).unwrap(),
        };

        let request = self
            .get("merge_requests")
            .query(&[("updated_after", from.to_string().as_str())]);
        info!("request: {:?}", request);
        let merge_requests = request
            .send()
            .await
            .unwrap()
            .json::<Vec<MergeRequest>>()
            .await
            .unwrap();
        info!("Number of MR: {:?}", merge_requests.len());

        let results = join_all(
            merge_requests
                .iter()
                .map(|merge_request| async {
                    let discussions = self
                        .get(&format!("merge_requests/{}/discussions", merge_request.iid))
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<MergeRequestDiscussion>>()
                        .await
                        .unwrap();
                    (merge_request.clone(), discussions)
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let mut change_requests = match cache {
            Some(cache) => cache.change_requests,
            None => vec![],
        };

        for (merge_request, discussions) in results {
            for discussion in discussions {
                if let Some(note) = discussion.notes.first() {
                    if note.system == false && note.author.id == 20796726 {
                        let parsed_note = ParsedNote::from(note.body.clone());
                        let change_request = ChangeRequest {
                            author: merge_request.author.username.clone(),
                            category: parsed_note.category,
                            description: parsed_note.description,
                            id: note.id,
                            merge_request_id: merge_request.iid,
                            sub_category: parsed_note.sub_category,
                            url: format!("{}/#note_{}", merge_request.web_url, note.id),
                        };
                        change_requests.push(change_request);
                    }
                }
            }
        }

        LocalStorage::set(
            "change_requests",
            Cache {
                change_requests: change_requests.clone(),
                from: Utc::now(),
            },
        )
        .ok();

        change_requests
    }

    // pub async fn save(&self, change_request: &ChangeRequest) {
    //     let parsed_note = ParsedNote::from(change_request);
    //     let endpoint = EditMergeRequestNote::builder()
    //         .project("archipels-managed/connect-monorepo")
    //         .merge_request(change_request.merge_request_id)
    //         .note(change_request.id)
    //         .body(parsed_note.to_string())
    //         .build()
    //         .unwrap();
    //     api::ignore(endpoint).query_async(&self.0).await.unwrap();
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cache {
    pub change_requests: Vec<ChangeRequest>,
    pub from: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
struct MergeRequest {
    pub author: Author,
    pub iid: u64,
    pub web_url: String,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequestDiscussion {
    pub notes: Vec<MergeRequestNote>,
}

#[derive(Debug, Deserialize)]
pub struct MergeRequestNote {
    pub author: Author,
    pub body: String,
    pub id: u64,
    pub system: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub id: u64,
    pub username: String,
}
