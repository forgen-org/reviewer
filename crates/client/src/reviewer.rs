use crate::{ai_client::AiClient, change_request::ChangeRequest, gitlab_client::GitlabClient};

pub struct Reviewer {
    ai_client: AiClient,
    gitlab_client: GitlabClient,
}

impl Reviewer {
    pub async fn new() -> Self {
        let ai_client = AiClient::new();
        let gitlab_client = GitlabClient::new().await;
        Self {
            ai_client,
            gitlab_client,
        }
    }

    pub async fn fetch(&self) -> Vec<ChangeRequest> {
        self.gitlab_client.fetch().await
    }

    pub async fn review(&self) {
        let change_requests = self.gitlab_client.fetch().await;
        for mut change_request in change_requests {
            match (&change_request.category, &change_request.sub_category) {
                (None, None) => {
                    self.ai_client.categorize(&mut change_request).await;
                    self.gitlab_client.save(&change_request).await;
                    println!("UPDATED: {:?}", change_request);
                }
                _ => {}
            }
        }
    }
}
