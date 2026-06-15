use std::sync::Arc;

use crate::infrastructure::email::azure_email_sender::AzureEmailSender;
use crate::infrastructure::email::email_sender::EmailService;

pub fn build_email_service() -> Arc<dyn EmailService> {
    Arc::new(AzureEmailSender::new())
}
