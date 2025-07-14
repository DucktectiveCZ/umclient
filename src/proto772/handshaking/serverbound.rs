use crate::proto772::encoding::{Error, Packet, Result};
use crate::proto772::handshaking::common::State;
use crate::proto772::util;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};
use varint_rs::{VarintReader, VarintWriter};

pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: State,
}

impl Packet for Handshake {
    const PACKET_ID: i32 = 0x00;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_i32_varint(self.protocol_version)
            .map_err(Error::VarintWriter)?;
        util::write_string(&self.server_address, writer)?;
        writer.write_all(&self.server_port.to_be_bytes())?;
        writer
            .write_i32_varint(self.next_state as i32)
            .map_err(Error::VarintWriter)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let protocol_version = reader.read_i32_varint().map_err(Error::VarintWriter)?;
        let server_address = util::read_string(reader)?;
        let server_port = reader.read_u16::<BigEndian>()?;
        let next_state = reader
            .read_i32_varint()
            .map_err(Error::VarintWriter)?
            .try_into()?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
