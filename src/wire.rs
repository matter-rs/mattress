use crate::{NodeID, GID};
use bitfield::bitfield;
use std::io;

bitfield! {
    pub struct MessageFlags(u8);
    impl Debug;
    pub version, set_version: 4, 0;
    pub has_source, set_has_source: 5;
    pub dsiz, set_dsiz: 7, 6;
}

bitfield! {
    pub struct SecurityFlags(u8);
    impl Debug;
    pub privacy, set_privacy: 0;
    pub control, set_control: 1;
    pub extensions, set_extensions: 2;
    pub session_type, set_session_type: 7, 6;
}

bitfield! {
    pub struct ExchangeFlags(u8);
    impl Debug;
    pub initiator, set_initiator: 7;
    pub ack, set_ack: 6;
    pub reliability, set_reliability: 5;
    pub secure_extensions, set_secure_extensions: 4;
    pub vendor, set_vendor: 3;
}

pub struct MessageHeader {
    message_flags: MessageFlags,
    session_id: u16,
    security_flags: SecurityFlags,
    message_counter: u32,
    source: Option<NodeID>,
    destination: Option<Destination>,
}

impl MessageHeader {
    pub fn write_raw<T: io::Write + Sized>(mut self, writer: &mut T) -> Result<(), io::Error> {
        writer.write_all(&self.message_flags.0.to_le_bytes())?;
        writer.write_all(&self.session_id.to_le_bytes())?;
        writer.write_all(&self.security_flags.0.to_le_bytes())?;
        writer.write_all(&self.message_counter.to_le_bytes())?;
        if let Some(source) = self.source {
            writer.write_all(&source.0.to_le_bytes())?;
        }
        match self.destination {
            Some(Destination::NodeID(node_id)) => writer.write_all(&node_id.0.to_le_bytes())?,
            Some(Destination::GID(gid)) => writer.write_all(&gid.0.to_le_bytes())?,
            None => {}
        }

        Ok(())
    }
}

struct MessageFrameOwned {
    header: MessageHeader,
    extensions: Vec<u8>,
    payload: Vec<u8>,
    footer: Vec<u8>,
}

pub enum Destination {
    NodeID(NodeID),
    GID(GID),
}
