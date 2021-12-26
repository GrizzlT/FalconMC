use crate::errors::*;

use falcon_core::network::buffer::PacketBufferRead;
use falcon_core::network::{ConnectionState, PacketHandlerState};
use falcon_core::player::MinecraftPlayer;
use falcon_core::server::Difficulty;
use falcon_core::world::chunks::Chunk;
use crate::implement_packet_handler_enum;
use crate::version::ProtocolVersioned;
use crate::version::v1_13::play::{ChunkDataPacket, JoinGamePacket, PlayerAbilitiesPacket, PlayerPositionAndLookPacket};
use crate::version::v1_13_2::login::LoginPackets;

pub mod login;

pub enum PacketList {
    Login(LoginPackets),
}

implement_packet_handler_enum!(PacketList, Login);

impl PacketList {
    pub fn from(
        packet_id: i32,
        state: &PacketHandlerState,
        buffer: &mut dyn PacketBufferRead,
    ) -> Result<Option<PacketList>> {
        match state.get_connection_state() {
            ConnectionState::Login => {
                LoginPackets::from(packet_id, buffer).map(|l| l.map(|p| PacketList::Login(p)))
            }
            _ => Ok(None),
        }
    }
}

pub struct PacketSend;

impl ProtocolVersioned for PacketSend {
    fn join_game(&self, player: &mut dyn MinecraftPlayer, difficulty: Difficulty, max_players: u8, level_type: String, reduced_debug: bool) -> Result<()> {
        let packet = JoinGamePacket::new(player.get_entity_id(), player.get_game_mode(), player.get_dimension(), difficulty, max_players, level_type, reduced_debug);
        player.get_client_connection().send(Box::new(move |conn| {
            let packet_out = packet;
            conn.send_packet(0x25, &packet_out);
        })).map_err(|_| Error::from("Could not send join game packet"))
    }

    fn player_abilities(&self, player: &mut dyn MinecraftPlayer, flying_speed: f32, fov_modifier: f32) -> Result<()> {
        let packet = PlayerAbilitiesPacket::new(player.get_ability_flags(), flying_speed, fov_modifier);
        player.get_client_connection().send(Box::new(move |conn| {
            let packet_out = packet;
            conn.send_packet(0x2E, &packet_out);
        })).map_err(|_| Error::from("Could not send player abilities packet"))
    }

    fn send_chunk(&self, player: &mut dyn MinecraftPlayer, chunk: &Chunk) -> Result<()> {
        let packet = ChunkDataPacket::from_chunk(chunk);
        player.get_client_connection().send(Box::new(move |conn| {
            let packet_out = packet;
            conn.send_packet(0x22, &packet_out);
        })).map_err(|_| Error::from("Could not send chunk data"))
    }

    fn player_position_and_look(&self, player: &mut dyn MinecraftPlayer, flags: u8, teleport_id: i32) -> Result<()> {
        let pos = player.get_position_copy();
        let look = player.get_look_angles_copy();
        let packet = PlayerPositionAndLookPacket::new(pos.get_x(), pos.get_y(), pos.get_z(), look.get_yaw(), look.get_pitch(), flags, teleport_id);
        player.get_client_connection().send(Box::new(move |conn| {
            let packet_out = packet;
            conn.send_packet(0x32, &packet_out);
        })).map_err(|_| Error::from("Could not send player position and look packet"))
    }
}
