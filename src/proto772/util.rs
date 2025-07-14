use std::io::{Read, Write};

use crate::proto772::encoding::{Decode, Encode, Error, Result};
use byteorder::ReadBytesExt;
use varint_rs::{VarintReader, VarintWriter};

pub fn read_varint_as_usize<R: Read>(reader: &mut R) -> Result<usize> {
    let value = reader.read_i32_varint()?;
    if value < 0 {
        Err(Error::Invalid("A string's length cannot be negative"))
    } else {
        Ok(value as usize)
    }
}

pub fn write_usize_as_varint<W: Write>(writer: &mut W, value: usize) -> Result<()> {
    if value > i32::MAX as usize {
        return Err(Error::IntOverflow(value))
    }

    writer.write_i32_varint(value as i32)?;

    Ok(())
}

pub fn read_bool<R: Read>(reader: &mut R) -> Result<bool> {
    match reader.read_u8()? {
        0x00 => Ok(false),
        0x01 => Ok(true),
        v => Err(Error::BadBool(v)),
    }
}

pub fn write_bool<W: Write>(value: bool, writer: &mut W) -> Result<()> {
    let byte_value = if value { 0x01 } else { 0x00 };
    writer.write_u8_varint(byte_value)?;
    Ok(())
}

pub fn write_string<W: Write>(s: &str, writer: &mut W) -> Result<()> {
    writer.write_i32_varint(s.len() as i32)?;
    writer.write_all(s.as_bytes())?;

    Ok(())
}

pub fn read_string<R: Read>(reader: &mut R) -> Result<String> {
    let len = read_varint_as_usize(reader)?;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

pub fn write_array<W: Write, T: Encode>(buf: &[T], writer: &mut W) -> Result<()> {
    for elem in buf {
        elem.proto772_encode(writer)?;
    }

    Ok(())
}

pub fn write_prefixed_array<W: Write, T: Encode>(buf: &[T], writer: &mut W) -> Result<()> {
    let len_usize = buf.len();
    if len_usize > i32::MAX as usize {
        return Err(Error::IntOverflow(len_usize));
    }
    let len = len_usize as i32;

    writer.write_i32_varint(len)?;

    write_array(buf, writer)?;

    Ok(())
}

pub fn read_prefixed_array<R: Read, T: Decode>(reader: &mut R) -> Result<Vec<T>> {
    let len = read_varint_as_usize(reader)?;
    let mut out = Vec::with_capacity(len);

    for _ in 0..len {
        let data = T::proto772_decode(reader)?;
        out.push(data);
    }

    Ok(out)
}

pub fn write_option<W: Write, T: Encode>(value: Option<&T>, writer: &mut W) -> Result<()> {
    let is_some = value.is_some();
    write_bool(is_some, writer)?;

    if let Some(val) = value {
        val.proto772_encode(writer)?;
    }

    Ok(())
}

pub fn read_option<R: Read, T: Decode>(reader: &mut R) -> Result<Option<T>> {
    let is_some = read_bool(reader)?;

    Ok(if is_some {
        let data = T::proto772_decode(reader)?;
        Some(data)
    } else {
        None
    })
}

pub fn write_option_string<W: Write>(s: Option<&str>, writer: &mut W) -> Result<()> {
    write_bool(s.is_some(), writer)?;

    if let Some(string) = s {
        write_string(string, writer)?;
    }

    Ok(())
}
