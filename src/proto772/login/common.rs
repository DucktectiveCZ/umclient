use crate::proto772::{
    encoding::{Decode, Encode, Result},
    util::{read_option, read_string, write_option, write_string},
};

pub struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}

impl Encode for Property {
    fn proto772_encode<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        write_string(&self.name, writer)?;
        write_string(&self.value, writer)?;
        write_option(self.signature.as_ref(), writer)?;

        Ok(())
    }
}

impl Decode for Property {
    fn proto772_decode<R: std::io::Read>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
    {
        let name = read_string(reader)?;
        let value = read_string(reader)?;
        let signature = read_option(reader)?;

        Ok(Self {
            name,
            value,
            signature,
        })
    }
}
