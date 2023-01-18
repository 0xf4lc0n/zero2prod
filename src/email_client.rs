use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    api_key: Secret<String>,
}

#[derive(serde::Serialize)]
struct SendEmailRequest<'a> {
    from: EmailPeer<'a>,
    subject: &'a str,
    content: Vec<EmailContent<'a>>,
    personalization: Vec<EmailPersonalization<'a>>,
}

#[derive(serde::Serialize)]
struct EmailPeer<'a> {
    email: &'a str,
    name: &'a str,
}

#[derive(serde::Serialize)]
struct EmailContent<'a> {
    r#type: &'a str,
    value: &'a str,
}

#[derive(serde::Serialize)]
struct EmailPersonalization<'a> {
    to: EmailPeer<'a>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        api_key: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        Self {
            http_client,
            base_url,
            sender,
            api_key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        // curl --request POST \
        // --url https://emailapi.netcorecloud.net/v5/mail/send \
        // --header 'api_key: <Your API Key>' \
        //     --header 'content-type: application/json' \
        //     --data '{
        // "from":{"email":"pax12@pepisandbox.com","name":"pax12"},
        // "subject":"Your Barcelona flight e-ticket : BCN2118050657714",
        // "content":[{"type":"html","value":"Hello Lionel, Your flight for Barcelona is confirmed."}],
        // "personalizations":[{"to":[{"email":"pax12@tuta.io","name":"Lionel Messi"}]}]}'

        // /v5/mail/send
        let url = format!("{}/v5/mail/send", self.base_url);

        let request_body = SendEmailRequest {
            from: EmailPeer {
                email: self.sender.as_ref(),
                name: "zero2prod".into(),
            },
            subject,
            content: vec![
                EmailContent {
                    r#type: "html",
                    value: html_content,
                },
                EmailContent {
                    r#type: "text",
                    value: text_content,
                },
            ],
            personalization: vec![EmailPersonalization {
                to: EmailPeer {
                    email: recipient.as_ref(),
                    name: recipient.as_ref(),
                },
            }],
        };

        self.http_client
            .post(&url)
            .header("api_key", self.api_key.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
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
    use secrecy::Secret;
    use wiremock::matchers::{any, header, method, path};
    use wiremock::Request;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

            if let Ok(body) = result {
                dbg!(&body);
                body.get("from").is_some()
                    && body.get("subject").is_some()
                    && body.get("content").is_some()
                    && body.get("personalization").is_some()
            } else {
                // If parsing failed, do not match the request
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_fires_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header("Content-Type", "application/json"))
            .and(path("/v5/mail/send"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_err!(outcome);
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(20),
        )
    }
}
