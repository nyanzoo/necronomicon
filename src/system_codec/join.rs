use std::io::{Read, Write};

use crate::{Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{JoinAck, Position};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Join {
    pub(crate) header: Header,
    pub(crate) position: Position,
}

impl Join {
    pub fn new(header: Header, position: Position) -> Self {
        assert_eq!(header.kind(), Kind::Join);

        Self { header, position }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn ack(self) -> JoinAck {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> JoinAck {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Join
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Join);

        let position = Position::decode(reader)?;

        Ok(Self { header, position })
    }
}

impl<W> Encode<W> for Join
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
    use crate::{system_codec::Position, tests::test_encode_decode_packet, Kind};

    use super::Join;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Join,
            Join {
                position: Position::Tail {
                    frontend: "fe".to_owned(),
                },
            }
        );
    }
}
