use mc_chat::ChatComponent;

pub trait ConnectionLogic {
    fn disconnect(reason: ChatComponent);
}
