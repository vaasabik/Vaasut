/// Событие в движке
pub enum Event {
    EntityCreated(u64),
    EntityDestroyed(u64),
    InputReceived,
}
