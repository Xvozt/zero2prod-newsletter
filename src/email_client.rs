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
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
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
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };
        let _builder = self
            .http_client
            .post(url)
            .header("X-Postmark-Server-Token", self.auth_token.expose_secret())
            .json(&request_body)
            .send()
            .await?;
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
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use reqwest::Url;
    use secrecy::SecretString;
    use wiremock::matchers::{MethodExactMatcher, header};
    use wiremock::matchers::{header_exists, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
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
}
