use crate::ondo_remote::CommandStatus;

impl Into<CommandStatus> for tonic::Status {
    fn into(self) -> CommandStatus {
        let code = self.code();
        let message = self.message();

        CommandStatus {
            code: code.into(),
            message: message.to_string(),
        }
    }
}
