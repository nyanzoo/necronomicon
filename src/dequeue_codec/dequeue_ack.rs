use std::io::{Read, Write};

use crate::{Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DequeueAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
    pub(crate) value: Vec<u8>,
}

impl<R> PartialDecode<R> for DequeueAck
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::DequeueAck);

        let response_code = u8::decode(reader)?;
        let value = Vec::<u8>::decode(reader)?;

        Ok(Self {
            header,
            response_code,
            value,
        })
    }
}

impl<W> Encode<W> for DequeueAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl Ack for DequeueAck {
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

    use super::DequeueAck;

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::DequeueAck,
            DequeueAck {
                response_code: SUCCESS,
                value: vec![1, 2, 3],
            }
        );
    }

    #[test]
    fn test_ack() {
        test_ack_packet!(
            Kind::DequeueAck,
            DequeueAck {
                response_code: SUCCESS,
                value: vec![1, 2, 3],
            }
        );
    }
}
