use pumpkin_plugin_api::events::FromIntoEvent;

pub struct PlayerWithdarwEvent;
impl FromIntoEvent for PlayerWithdarwEvent {
    const EVENT_TYPE: EventType;

    type Data;

    fn data_from_event(event: Event) -> Self::Data {
        todo!()
    }

    fn data_into_event(data: Self::Data) -> Event {
        todo!()
    }
}
