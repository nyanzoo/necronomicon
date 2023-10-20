use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DeleteAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R> PartialDecode<R> for DeleteAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::DeleteQueueAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for DeleteAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for DeleteAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tests::{test_ack_packet, test_encode_decode_packet},
        Kind, SUCCESS,
    };

    use super::DeleteAck;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::DeleteQueueAck,
            DeleteAck {
                response_code: SUCCESS
            }
        );
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(
            Kind::DeleteQueueAck,
            DeleteAck {
                response_code: SUCCESS
            }
        );
    }
}
