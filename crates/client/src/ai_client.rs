use convert_case::{Case, Casing};
use openai_dive::v1::{
    api::Client,
    models::Gpt4Engine,
    resources::chat::{
        ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage,
        ChatMessageContent, JsonSchemaBuilder,
    },
};

use crate::{change_request::ChangeRequest, parsed_note::ParsedNote};

pub struct AiClient(Client);

impl AiClient {
    pub fn new() -> Self {
        Self(Client::new_from_env())
    }

    pub async fn categorize(&self, change_request: &mut ChangeRequest) {
        let instructions = r#"
            You are a helpful assistant that categorizes a change request.
            Categories and sub-categories must be in kebab-case.

            Possible categories:
                - effect: Anything that is related to the Effect-ts library.
                - oversight: A human error like a typo or a missing word.
                - domain: A misrepresentation of the domain or an error in the business logic.
                - complicated: A complicated function or data-structure that is hard to understand.
                - performance: A performance issue that can be solved by optimizing the code.
                - security: A security issue that can be solved by fixing the code.
                - documentation: A documentation issue that can be solved by fixing the code.
                - testing: A testing issue that can be solved by fixing the code.
                - other: Any other category that does not fit into the above categories.
            "#;

        let client = self.0.clone();

        let parameters = ChatCompletionParametersBuilder::default()
            .model(Gpt4Engine::Gpt4O.to_string())
            .messages(vec![
                ChatMessage::System {
                    content: ChatMessageContent::Text(instructions.to_string()),
                    name: None,
                },
                ChatMessage::User {
                    content: ChatMessageContent::Text(change_request.description.clone()),
                    name: None,
                },
            ])
            .response_format(ChatCompletionResponseFormat::JsonSchema(
                JsonSchemaBuilder::default()
                    .name("categories")
                    .schema(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "description": { "type": "string" },
                            "category": {
                                "type": "string",
                                "enum": [
                                    "effect",
                                    "oversight",
                                    "domain",
                                    "complicated",
                                    "performance",
                                    "security",
                                    "documentation",
                                    "testing",
                                    "other"
                                ]
                            },
                            "sub_category": { "type": "string" },
                        },
                        "required": ["description", "category", "sub_category"],
                        "additionalProperties": false
                    }))
                    .strict(true)
                    .build()
                    .unwrap(),
            ))
            .build()
            .unwrap();

        let result = client.chat().create(parameters).await.unwrap();

        let message = &result.choices.first().unwrap().message;

        if let ChatMessage::Assistant {
            content: Some(ChatMessageContent::Text(message)),
            ..
        } = message
        {
            let parsed_note: ParsedNote = serde_json::from_str(&message).unwrap();
            change_request.category = parsed_note.category.map(|v| v.to_case(Case::Kebab));
            change_request.sub_category = parsed_note.sub_category.map(|v| v.to_case(Case::Kebab));
        }
    }
}
