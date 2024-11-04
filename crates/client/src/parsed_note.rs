use std::fmt;

use regex::Regex;
use serde::Deserialize;

use crate::change_request::ChangeRequest;

#[derive(Debug, Deserialize)]
pub struct ParsedNote {
    pub description: String,
    pub category: Option<String>,
    pub sub_category: Option<String>,
}

impl From<String> for ParsedNote {
    fn from(str: String) -> Self {
        let str = str.trim();
        let re = Regex::new(r"(?U)^([\s\S]*)(\#([A-z-_]+)\/([A-z-_]+))?$").unwrap();
        let caps = re.captures(str).unwrap();

        let description = caps.get(1).unwrap().as_str().to_string();
        let category = caps.get(3).map(|m| m.as_str().to_string());
        let sub_category = caps.get(4).map(|m| m.as_str().to_string());

        ParsedNote {
            description,
            category,
            sub_category,
        }
    }
}

impl From<&ChangeRequest> for ParsedNote {
    fn from(change_request: &ChangeRequest) -> Self {
        ParsedNote {
            description: change_request.description.clone(),
            category: change_request.category.clone(),
            sub_category: change_request.sub_category.clone(),
        }
    }
}

impl fmt::Display for ParsedNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.description, &self.category, &self.sub_category) {
            (description, Some(category), None) => {
                write!(f, "{}  \n#{}/other", description, category)
            }
            (description, Some(category), Some(sub_category)) => {
                write!(f, "{}  \n#{}/{}", description, category, sub_category)
            }
            (description, _, _) => write!(f, "{}", description),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_request_note_into() {
        let parsed = ParsedNote::from("comment".to_string());
        assert!(parsed.description == "comment");
        assert!(parsed.category == None);
        assert!(parsed.sub_category == None);

        let parsed = ParsedNote::from(
            r#"comment
            #category/sub_category"#
                .to_string(),
        );
        assert!(parsed.description == "comment\n            ");
        assert!(parsed.category == Some("category".to_string()));
        assert!(parsed.sub_category == Some("sub_category".to_string()));

        let parsed = ParsedNote::from("added 1 commit\n\n<ul><li>655de802 - fix: various fix and improvements</li></ul>\n\n[Compare with previous version](/archipels-managed/connect-monorepo/-/merge_requests/1317/diffs?diff_id=1170847463&start_sha=22dde424204c6a05cfd3fc11c7958391fbe5a12a)".to_string());
        assert!(parsed.description == "added 1 commit\n\n<ul><li>655de802 - fix: various fix and improvements</li></ul>\n\n[Compare with previous version](/archipels-managed/connect-monorepo/-/merge_requests/1317/diffs?diff_id=1170847463&start_sha=22dde424204c6a05cfd3fc11c7958391fbe5a12a)");
        assert!(parsed.category == None);
        assert!(parsed.sub_category == None);
    }
}
