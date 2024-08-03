use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, Owned, Shared},
    Ack, Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PeekAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response_code: u8,
    pub(crate) value: Option<BinaryData<S>>,
}

impl<R, O> PartialDecode<R, O> for PeekAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::PeekAck);

        let response_code = u8::decode(reader)?;
        let value = Option::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            response_code,
            value,
        })
    }
}

impl<W, S> Encode<W> for PeekAck<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response_code.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack for PeekAck<S>
where
    S: Shared,
{
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
        buffer::{BinaryData, SharedImpl},
        tests::verify_encode_decode,
        Header, Kind, Packet, SUCCESS,
    };

    use super::PeekAck;

    impl PeekAck<SharedImpl> {
        pub fn new(response_code: u8, value: Option<BinaryData<SharedImpl>>) -> Self {
            PeekAck {
                header: Header::new(Kind::PeekAck, 1, 1, value.as_ref().map_or(0, |v| v.len())),
                response_code,
                value,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::PeekAck(PeekAck::new(SUCCESS, None)));
    }
}
