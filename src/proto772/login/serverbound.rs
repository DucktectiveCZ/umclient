use std::io::{Read, Write};
use varint_rs::{VarintReader, VarintWriter};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::proto772::{
    encoding::{Packet, Result},
    util::{
        read_option, read_prefixed_array, read_string, write_option, write_prefixed_array,
        write_string,
    },
};

pub struct LoginStart {
    pub name: String,
    pub player_uuid: u128,
}

impl Packet for LoginStart {
    const PACKET_ID: i32 = 0x00;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.name, writer)?;
        writer.write_u128::<BigEndian>(self.player_uuid)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let name = read_string(reader)?;
        let player_uuid = reader.read_u128::<BigEndian>()?;

        Ok(Self { name, player_uuid })
    }
}

pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl Packet for EncryptionResponse {
    const PACKET_ID: i32 = 0x01;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_prefixed_array(&self.shared_secret, writer)?;
        write_prefixed_array(&self.verify_token, writer)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let shared_secret = read_prefixed_array(reader)?;
        let verify_token = read_prefixed_array(reader)?;

        Ok(Self {
            shared_secret,
            verify_token,
        })
    }
}

pub struct LoginPluginResponse {
    message_id: i32,
    data: Option<Vec<u8>>,
}

impl Packet for LoginPluginResponse {
    const PACKET_ID: i32 = 0x02;

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let message_id = reader.read_i32_varint()?;
        let data = read_option(reader)?;

        Ok(Self { message_id, data })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32_varint(self.message_id)?;
        write_option(self.data.as_ref(), writer)?;

        Ok(())
    }
}

pub struct LoginAcknowledged;

impl Packet for LoginAcknowledged {
    const PACKET_ID: i32 = 0x03;

    fn encode_payload<W: Write>(&self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn decode_payload<R: std::io::Read>(_: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

pub struct CookieResponse {
    key: String,
    payload: Option<Vec<u8>>,
}

impl Packet for CookieResponse {
    const PACKET_ID: i32 = 0x04;

    fn decode_payload<R: std::io::Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let key = read_string(reader)?;
        let payload = read_option(reader)?;

        Ok(Self { key, payload })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.key, writer)?;
        write_option(self.payload.as_ref(), writer)?;

        Ok(())
    }
}

