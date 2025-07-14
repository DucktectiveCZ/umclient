use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use varint_rs::{VarintReader, VarintWriter};

use crate::proto772::{
    encoding::{Packet, Result},
    login::common::Property,
    util::{
        self, read_bool, read_prefixed_array, read_string, write_array, write_bool,
        write_prefixed_array, write_string,
    },
};

pub struct Disconnect {
    pub reason: String,
}

impl Packet for Disconnect {
    const PACKET_ID: i32 = 0x00;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        util::write_string(&self.reason, writer)?;
        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let reason = util::read_string(reader)?;

        Ok(Self { reason })
    }
}

pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: String,
    pub verify_token: String,
    pub should_authenticate: bool,
}

impl Packet for EncryptionRequest {
    const PACKET_ID: i32 = 0x01;

    fn decode_payload<R: std::io::Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let server_id = read_string(reader)?;
        let public_key = read_string(reader)?;
        let verify_token = read_string(reader)?;
        let should_authenticate = read_bool(reader)?;

        Ok(Self {
            server_id,
            public_key,
            verify_token,
            should_authenticate,
        })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.server_id, writer)?;
        write_string(&self.public_key, writer)?;
        write_string(&self.verify_token, writer)?;
        write_bool(self.should_authenticate, writer)?;

        Ok(())
    }
}

pub struct LoginSuccess {
    pub uuid: u128,
    pub username: String,
    pub name: String,
    pub property: Vec<Property>,
}

impl Packet for LoginSuccess {
    const PACKET_ID: i32 = 0x02;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u128::<BigEndian>(self.uuid)?;
        write_string(&self.username, writer)?;
        write_string(&self.name, writer)?;
        write_prefixed_array(&self.property, writer)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let uuid = reader.read_u128::<BigEndian>()?;
        let username = read_string(reader)?;
        let name = read_string(reader)?;
        let property = read_prefixed_array(reader)?;

        Ok(Self {
            uuid,
            username,
            name,
            property,
        })
    }
}

pub struct SetCompression {
    pub threshold: i32,
}

impl Packet for SetCompression {
    const PACKET_ID: i32 = 0x03;

    fn decode_payload<R: std::io::Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let treshold = reader.read_i32_varint()?;

        Ok(Self {
            threshold: treshold,
        })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32_varint(self.threshold)?;
        Ok(())
    }
}

pub struct LoginPluginRequest {
    pub message_id: i32,
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for LoginPluginRequest {
    const PACKET_ID: i32 = 0x04;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_i32_varint(self.message_id)?;
        write_string(&self.channel, writer)?;
        write_array(&self.data, writer)?;

        Ok(())
    }

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let message_id = reader.read_i32_varint()?;
        let channel = read_string(reader)?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            message_id,
            channel,
            data,
        })
    }
}

pub struct CookieRequest {
    pub key: String,
}

impl Packet for CookieRequest {
    const PACKET_ID: i32 = 0x05;

    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let key = read_string(reader)?;

        Ok(Self { key })
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.key, writer)?;

        Ok(())
    }
}
