use domain::submission_request::{ReviewSubmission, SubmissionRequest};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

async fn submit_review(request: SubmissionRequest) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://stg.api.bazaarvoice.com/data/submitreview.json")
        .header("X-Api-Version", "5.4")
        .json(&request)
        .send()
        .await?;
    println!("Response: {}", res.status());

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.json::<HashMap<String, bool>>().await?;
            println!("Body: {:#?}", body);
            if body["HasErrors"] {
                println!("Errors: {:#?}", body["FormErrors"]);
                return Err(Error::new(reqwest::StatusCode::BAD_REQUEST, "Bad request"));
            }

            return Ok((body));
        }
        _ => {
            return Err(Error::new(reqwest::StatusCode::BAD_REQUEST, "Bad request"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_submit_review_success() {
        let _m = mock("POST", "/submitreview.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"HasErrors": false}"#)
            .create();

        let review = ReviewSubmission {
            rating: 5,
            review_text: "Great product".into(),
            title: "Review title".into(),
            user_nickname: "User".into(),
            user_email: "user@example.com".into(),
            send_email_alert_when_published: false,
            reviewer_location: "Location".into(),
            product_id: "123".into(),
        };
        let request = SubmissionRequest::new("123", review);
        let result = submit_review(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_submit_review_error() {
        let _m = mock("POST", "/submitreview.json")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"HasErrors": true, "FormErrors": {"FieldErrors": {"rating": [{"Message": "Invalid rating"}]}}}"#)
            .create();

        let review = ReviewSubmission {
            rating: 5,
            review_text: "Great product".into(),
            title: "Review title".into(),
            user_nickname: "User".into(),
            user_email: "user@example.com".into(),
            send_email_alert_when_published: false,
            reviewer_location: "Location".into(),
            product_id: "123".into(),
        };
        let request = SubmissionRequest::new("123", review);
        let result = submit_review(request).await;
        assert!(result.is_err());
    }
}
