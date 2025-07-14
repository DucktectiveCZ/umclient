use std::io::{Read, Write};

use crate::proto772::{
    encoding::{Packet, Result},
    util::{read_string, write_string},
};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct StatusResponse {
    pub json_response: String,
}

impl Packet for StatusResponse {
    const PACKET_ID: i32 = 0x00;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.json_response, writer)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            json_response: read_string(reader)?,
        })
    }
}

pub struct PongResponse {
    pub timestamp: i64,
}

impl Packet for PongResponse {
    const PACKET_ID: i32 = 0x01;

    fn decode_payload<R: std::io::Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let timestamp = reader.read_i64::<BigEndian>()?;

        Ok(Self { timestamp })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i64::<BigEndian>(self.timestamp)?;

        Ok(())
    }
}
