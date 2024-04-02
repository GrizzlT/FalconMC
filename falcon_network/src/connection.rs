use bytes::Buf;
use falcon_core::{ShutdownHandle, network::{NetworkState, Phase}};
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use tracing::debug_span;

use crate::{McWriter, McReader};

mod logic;
pub use logic::*;

#[derive(Debug)]
pub struct FalconConnection {
    shutdown: ShutdownHandle,
    writer: McWriter,
    state: NetworkState,
}

impl FalconConnection {
    pub fn new(shutdown: ShutdownHandle, protocol: i32) -> Self {
        Self {
            shutdown,
            writer: McWriter::new(None),
            state: NetworkState::new(protocol),
        }
    }

    pub fn state(&self) -> &NetworkState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut NetworkState {
        &mut self.state
    }

    #[tracing::instrument(name = "connection", skip_all)]
    pub async fn start(mut self, mut socket: TcpStream) {
        let (mut readhalf, mut writehalf) = socket.split();
        let mut reader = McReader::new(false, true);

        loop {
            tokio::select! {
                _ = self.shutdown.wait_for_shutdown() => {
                    break;
                }

                // writing
                res = writehalf.write_all_buf(&mut self.writer), if self.writer.has_remaining() => {
                    if res.is_err() {
                        self.state.phase = Phase::Disconnected;
                        break;
                    } else if !self.writer.has_remaining() && self.state.phase == Phase::Disconnected {
                        break;
                    }
                }

                // reading
                n = readhalf.read_buf(&mut reader) => {
                    let span = debug_span!("Incoming", state = %self.state);
                    let _enter = span.enter();
                    match n {
                        Ok(n) => {
                            // other end closed the connection
                            if n == 0 {
                                self.state.phase = Phase::Disconnected;
                                break;
                            }
                            // iterate over all received packets
                            while let Some(_packet) = reader.next_packet().transpose() {
                                // TODO: parse packet
                            }

                            if !self.writer.has_remaining() && self.state.phase == Phase::Disconnected {
                                break;
                            }
                        }
                        Err(err) => {
                            tracing::warn!(error = %err);
                            // TODO: disconnect
                        }
                    }
                }
            }
        }
    }
}
