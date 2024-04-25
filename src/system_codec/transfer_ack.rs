use std::io::{Read, Write};

use crate::{buffer::Owned, Ack, Decode, Encode, Error, Header, Kind, PartialDecode};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct TransferAck {
    pub(crate) header: Header,
    pub(crate) response_code: u8,
}

impl<R, O> PartialDecode<R, O> for TransferAck
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::TransferAck);

        let response_code = u8::decode(reader)?;

        Ok(Self {
            header,
            response_code,
        })
    }
}

impl<W> Encode<W> for TransferAck
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;

        Ok(())
    }
}

impl Ack for TransferAck {
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::verify_encode_decode, Header, Kind, Packet, SUCCESS};

    use super::TransferAck;

    impl TransferAck {
        pub fn new(response_code: u8) -> Self {
            Self {
                header: Header::new_test_ack(Kind::TransferAck),
                response_code,
            }
        }
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::TransferAck(TransferAck::new(SUCCESS)));
    }
}
