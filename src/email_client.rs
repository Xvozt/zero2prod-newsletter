use crate::domain::SubscriberEmail;
use reqwest::{Client, Url};

pub struct EmailClient {
    sender: SubscriberEmail,
    base_url: Url,
    http_client: Client,
}

impl EmailClient {
    pub async fn send_mail(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        let url = self.base_url.join("/email").unwrap();
        let builder = self.http_client.post(url);
        Ok(())
    }

    pub fn new(base_url: Url, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;
    use reqwest::Url;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let url = Url::parse(&mock_server.uri()).expect("Failed to parse server url");
        let email_client = EmailClient::new(url, sender);
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let subscriber_mail = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client.send_mail(subscriber_mail, &subject, &content, &content).await;

        // Assert
    }
}