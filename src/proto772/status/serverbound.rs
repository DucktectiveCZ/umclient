use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::proto772::encoding::{Packet, Result};

pub struct StatusRequest {}

impl Packet for StatusRequest {
    const PACKET_ID: i32 = 0x00;

    fn encode_payload<W: Write>(&self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn decode_payload<R: Read>(_: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

pub struct PingRequest {
    pub timestamp: i64,
}

impl Packet for PingRequest {
    const PACKET_ID: i32 = 0x01;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i64::<BigEndian>(self.timestamp)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let timestamp = reader.read_i64::<BigEndian>()?;

        Ok(Self { timestamp })
    }
}
