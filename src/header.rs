use std::{
    io::{Read, Write},
    mem::size_of,
};

use crate::{error::Error, Decode, Encode};

// TODO: need to map these to packet types, also need to do partial
// decodes of header to get packet type and then decode the rest.
pub enum Kind {

}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Header {
    kind: u8,
    version: u8,
    uuid: u128,
}

impl Header {
    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn uuid(&self) -> u128 {
        self.uuid
    }
}

impl Decode for Header {
    fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let mut bytes = [0; size_of::<Header>()];
        reader
            .read_exact(&mut bytes)
            .map_err(Error::IncompleteHeader)?;

        let mut header = Header {
            kind: 0,
            version: 0,
            uuid: 0,
        };

        // Todo: verify kind
        header.kind = bytes[0];
        // Todo: verify version
        header.version = bytes[1];
        // Todo: verify length?
        header.uuid = u128::from_be_bytes(bytes[2..18].try_into().expect("not u64"));

        Ok(header)
    }
}

impl Encode for Header {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut buf = [0; size_of::<Header>()];
        buf[0] = self.kind;
        buf[1] = self.version;
        buf[2..18].copy_from_slice(&self.uuid.to_be_bytes());

        writer.write_all(&buf).map_err(Error::Encode)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use test_case::test_case;

    use crate::{Decode, Encode};

    use super::Header;

    #[test_case(0, 0, 0; "zero")]
    #[test_case(1, 1, 1; "one")]
    #[test_case(2, 2, 2; "two")]
    fn test_header_encode_decode(kind: u8, version: u8, uuid: u128) {
        let mut buf = Vec::new();
        let header = Header {
            kind,
            version,
            uuid,
        };
        header.encode(&mut buf).expect("encode");

        let mut buf = Cursor::new(buf);
        let actual = Header::decode(&mut buf).expect("decode");
        assert_eq!(actual, header);
    }
}
