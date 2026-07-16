#[derive(Clone, Debug, Default)]
pub struct MockNotificationProvider {
    messages: Vec<server_runtime::NotificationMessage>,
}

impl MockNotificationProvider {
    #[must_use]
    pub fn messages(&self) -> &[server_runtime::NotificationMessage] {
        &self.messages
    }

    pub fn send(&mut self, message: server_runtime::NotificationMessage) {
        self.messages.push(message);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemoteSyncRequestCount(usize);

impl From<RemoteSyncRequestCount> for usize {
    fn from(value: RemoteSyncRequestCount) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteSyncBytes(Vec<u8>);

impl From<Vec<u8>> for RemoteSyncBytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for RemoteSyncBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

#[derive(Clone, Debug)]
pub struct RemoteSyncSource {
    bytes: RemoteSyncBytes,
    request_count: RemoteSyncRequestCount,
}

impl RemoteSyncSource {
    #[must_use]
    pub const fn new(bytes: RemoteSyncBytes) -> Self {
        Self {
            bytes,
            request_count: RemoteSyncRequestCount(0usize),
        }
    }

    #[must_use]
    pub const fn read(&mut self) -> &RemoteSyncBytes {
        self.request_count.0 = self.request_count.0.saturating_add(1usize);
        &self.bytes
    }

    #[must_use]
    pub const fn request_count(&self) -> RemoteSyncRequestCount {
        self.request_count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn remote_source_returns_configured_payload_and_counts_reads() {
        let mut source = super::RemoteSyncSource::new(super::RemoteSyncBytes::from(vec![1u8, 2u8]));
        assert_eq!(source.read().as_ref(), &[1u8, 2u8]);
        assert_eq!(usize::from(source.request_count()), 1usize);
    }

    #[test]
    fn notification_provider_records_messages() {
        let mut provider = super::MockNotificationProvider::default();
        let message = server_runtime::NotificationMessage::try_from(
            str_constants::TEST_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect("6ef25d4a");
        provider.send(message);
        assert_eq!(provider.messages().len(), 1usize);
    }
}
