use serde::{Deserialize, Serialize};
use validator::Validate;

// Our generic review struct for incoming requests, which is then transformed
// into implementation specific requests.
#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    #[validate(range(min = 0, max = 5))]
    pub rating: u8,
    #[validate(length(min = 1))]
    pub review_text: String,
    pub title: String,
    pub user_nickname: String,
    #[validate(email)]
    pub user_email: String,
    pub product_id: String,
}

// This is the impl specific for Review requests to BV's conversations API
// See: https://developer.bazaarvoice.com/conversations-api/reference/v5.4/reviews/review-submission/full-submission
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubmissionRequest {
    pub api_version: String,
    pub action: String,
    pub campaign_id: String,

    // Fingerprint of content author's device.
    #[serde(rename = "fp")]
    pub fingerprint: String,
    pub locale: String,
    pub product_id: String,
    pub rating: u8,
    pub review_text: String,
    pub title: String,
    pub user_email: String,
    #[serde(rename = "UserNickname")]
    pub user_display_name: String,

    #[serde(rename = "PhotoUrl", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

impl SubmissionRequest {
    pub fn new(product_id: &str, review: Review) -> Self {
        Self {
            api_version: "5.4".to_string(),
            action: "Submit".to_string(),
            product_id: product_id.to_string(),
            review_text: review.review_text,
            rating: 0,
            photo_url: None,
        }
    }
}
