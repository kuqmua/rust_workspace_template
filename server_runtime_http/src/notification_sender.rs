pub trait NotificationSender: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn send(
        &self,
        message: super::NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
