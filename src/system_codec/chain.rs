use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{ChainAck, Position};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Chain {
    pub(crate) header: Header,
    pub(crate) position: Position,
}

impl Chain {
    pub fn new(header: Header, position: Position) -> Self {
        assert_eq!(header.kind(), Kind::Chain);

        Self { header, position }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn ack(self) -> ChainAck {
        ChainAck {
            header: Header::new(Kind::ChainAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> ChainAck {
        ChainAck {
            header: Header::new(Kind::ChainAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Chain
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Chain);

        let position = Position::decode(reader)?;

        Ok(Self { header, position })
    }
}

impl<W> Encode<W> for Chain
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.position.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{system_codec::Position, Decode, Encode, Header, Kind, PartialDecode};

    use super::Chain;

    #[test]
    fn test_encode_decode() {
        let header = Header::new(Kind::Chain, 123, 456);
        let mut buf = Vec::new();
        let chain = Chain {
            header,
            position: Position::Head {
                next: "next".to_owned(),
            },
        };
        chain.encode(&mut buf).unwrap();
        let mut buf = buf.as_slice();
        let header = Header::decode(&mut buf).unwrap();
        let decoded = Chain::decode(header, &mut buf).unwrap();
        assert_eq!(chain, decoded);
    }
}
