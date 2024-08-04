use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    response::Response,
    Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode,
};

use super::PeekAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Peek<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
    pub(crate) sequence: u64,
}

impl<S> Peek<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        path: ByteStr<S>,
        sequence: u64,
    ) -> Self {
        Self {
            header: Header::new(Kind::Peek, version, uuid, path.len()),
            path,
            sequence,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn ack(self, value: BinaryData<S>) -> PeekAck<S> {
        PeekAck {
            header: Header::new(
                Kind::PeekAck,
                self.header.version,
                self.header.uuid,
                value.len(),
            ),
            response: Response::success(),
            value: Some(value),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> PeekAck<S> {
        PeekAck {
            header: Header::new(Kind::PeekAck, self.header.version, self.header.uuid, 0),
            response: Response::fail(response_code, reason),
            value: None,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Peek<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Peek);

        let path = ByteStr::decode_owned(reader, buffer)?;
        let sequence = u64::decode(reader)?;

        Ok(Self {
            header,
            path,
            sequence,
        })
    }
}

impl<W, S> Encode<W> for Peek<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.sequence.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::{binary_data, byte_str},
        tests::verify_encode_decode,
        Ack, Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Peek;

    #[test]
    fn acks() {
        let peek = Peek::new(1, 2, byte_str(b"test"), 0);

        let ack = peek.clone().ack(binary_data(&[1, 2, 3]));
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = peek.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Peek(Peek::new(1, 1, byte_str(b"test"), 0)));
    }
}
