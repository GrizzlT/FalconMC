use std::fmt::Debug;
use std::net::SocketAddr;
use mc_chat::ChatComponent;

use falcon_core::network::PacketHandlerState;
use falcon_core::network::packet::PacketEncode;

pub trait ConnectionLogic: Debug {
    fn address(&self) -> &SocketAddr;

    fn handler_state(&self) -> &PacketHandlerState;

    fn handler_state_mut(&mut self) -> &mut PacketHandlerState;

    fn send_packet<P: PacketEncode>(&mut self, packet_id: i32, data: P);

    fn send<P: PacketEncode>(&mut self, data: P);

    fn disconnect(&mut self, reason: ChatComponent);
}

