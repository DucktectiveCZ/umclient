use std::{
    io::{self, Read, Write},
    string::FromUtf8Error,
    u8,
};

use byteorder::{ReadBytesExt, WriteBytesExt};
use varint_rs::VarintWriter;

use crate::proto772::util::{read_bool, read_varint_as_usize, write_bool, write_usize_as_varint};

type VarintWriteError = <Vec<u8> as VarintWriter>::Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Varint write error: {0}")]
    VarintWriter(VarintWriteError),

    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("Invalid packet: {0}")]
    Invalid(&'static str),

    #[error("Utf-8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Invalid state: {0}")]
    BadState(i32),

    #[error("Invalid value: {0}")]
    BadBool(u8),

    #[error("Value too big for an int32: {0}")]
    IntOverflow(usize),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Packet {
    const PACKET_ID: i32;

    fn encode_payload<W: Write>(&self, writer: &mut W) -> Result<()>;
    fn decode_payload<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized;
}

pub trait Encode {
    fn proto772_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}

impl Encode for String {
    fn proto772_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        write_usize_as_varint(writer, self.len())?;
        writer.write(self.as_bytes())?;

        Ok(())
    }
}

impl<T: Encode> Encode for Option<T> {
    fn proto772_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let is_some = self.is_some();

        write_bool(is_some, writer)?;

        if let Some(val) = self {
            val.proto772_encode(writer)?;
        }

        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn proto772_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        for elem in self {
            elem.proto772_encode(writer)?;
        }

        Ok(())
    }
}

impl Encode for u8 {
    fn proto772_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(*self)?;

        Ok(())
    }
}

pub trait Decode {
    fn proto772_decode<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Decode> Decode for Option<T> {
    fn proto772_decode<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let is_some = read_bool(reader)?;
        Ok(if is_some {
            let data = T::proto772_decode(reader)?;
            Some(data)
        } else {
            None
        })
    }
}

impl Decode for String {
    fn proto772_decode<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let len = read_varint_as_usize(reader)?;

        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;

        Ok(String::from_utf8(buf)?)
    }
}

impl Decode for u8 {
    fn proto772_decode<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(reader.read_u8()?)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn proto772_decode<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let len = read_varint_as_usize(reader)?;

        let mut buf = Vec::new();

        for _ in 0..len {
            let data = T::proto772_decode(reader)?;
            buf.push(data);
        }

        Ok(buf)
    }
}

pub struct RawPacket {
    pub packet_id: i32,
    pub payload: Vec<u8>,
}

