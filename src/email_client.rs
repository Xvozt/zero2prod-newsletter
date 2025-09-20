use crate::domain::SubscriberEmail;
use reqwest::{Client, Url};
use secrecy::{ExposeSecret, SecretString};

pub struct EmailClient {
    sender: SubscriberEmail,
    base_url: Url,
    http_client: Client,
    auth_token: SecretString,
}
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub async fn send_mail(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self.base_url.join("/email").unwrap();
        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject: subject,
            html_body: html_content,
            text_body: text_content,
        };
        let _builder = self
            .http_client
            .post(url)
            .header("X-Postmark-Server-Token", self.auth_token.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub fn new(base_url: Url, sender: SubscriberEmail, auth_token: SecretString) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            auth_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use reqwest::Url;
    use secrecy::SecretString;
    use wiremock::matchers::{MethodExactMatcher, any, header};
    use wiremock::matchers::{header_exists, path};
    use wiremock::{Match, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_sends_expected_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let url = Url::parse(&mock_server.uri()).expect("Failed to parse server url");
        let email_client =
            EmailClient::new(url, sender, SecretString::from(Faker.fake::<String>()));
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(MethodExactMatcher::new("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_mail = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_mail(subscriber_mail, &subject, &content, &content)
            .await;

        // Assert
    }
    #[tokio::test]
    async fn send_email_succeeds_when_server_returns_200() {
        //Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let url = Url::parse(&mock_server.uri()).expect("Failed to parse server url");
        let email_client =
            EmailClient::new(url, sender, SecretString::from(Faker.fake::<String>()));
        let subscriber_mail = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        //Act
        let outcome = email_client
            .send_mail(subscriber_mail, &subject, &content, &content)
            .await;

        //Assert
        assert_ok!(outcome);
    }
    #[tokio::test]
    async fn send_email_fails_if_server_returns_500() {
        //Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let url = Url::parse(&mock_server.uri()).expect("Failed to parse server url");
        let email_client =
            EmailClient::new(url, sender, SecretString::from(Faker.fake::<String>()));
        let subscriber_mail = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        //Act
        let outcome = email_client
            .send_mail(subscriber_mail, &subject, &content, &content)
            .await;

        //Assert
        assert_err!(outcome);
    }
    struct SendEmailBodyMatcher;

    impl Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }
}
