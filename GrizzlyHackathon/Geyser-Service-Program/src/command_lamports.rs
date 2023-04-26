use geyser_service_common::ServiceCommand;

pub fn service_cost(command: &ServiceCommand) -> u64 {
    match command {
        ServiceCommand::DecentralizedNotification { .. } => 2_500_000,
        ServiceCommand::NewsletterEmail { .. } => 1_500_000,
        _=> 2_000_000,
    }
}
