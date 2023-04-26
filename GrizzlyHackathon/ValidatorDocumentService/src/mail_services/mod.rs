use xor_mailer::{Mailer, MailerConfig};
use xor_mailer_common::Envelope;

#[derive(Debug, Default)]
pub struct MailCreator {
    recipient_name: String,
    recipient_email: String,
    subject: String,
    body: String,
}

impl MailCreator {
    pub fn new() -> Self {
        MailCreator::default()
    }

    pub fn add_recipient_name(mut self, name: &str) -> Self {
        self.recipient_name = name.to_owned();

        self
    }

    pub fn add_recipient_email(mut self, email: &str) -> Self {
        self.recipient_email = email.to_owned();

        self
    }

    pub fn add_subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_owned();

        self
    }

    pub fn add_body(mut self, body: &str) -> Self {
        self.body = body.to_owned();

        self
    }

    pub async fn send(self, config: &MailerConfig) {
        let mut envelope = Envelope::new();
        envelope
            .add_recipient((&self.recipient_name, &self.recipient_email))
            .add_subject(&self.subject)
            .add_html_body(&self.body);

        Mailer::new(config)
            .add_envelope(envelope)
            .send()
            .await
            .unwrap();
    }
}
