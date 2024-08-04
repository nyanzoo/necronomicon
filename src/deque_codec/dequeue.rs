use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    response::Response,
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode,
};

use super::DequeueAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Dequeue<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
}

impl<S> Dequeue<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, path: ByteStr<S>) -> Self {
        Self {
            header: Header::new(Kind::Deque, version, uuid, path.len()),
            path,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn ack<S1>(self, value: BinaryData<S1>) -> DequeueAck<S1>
    where
        S1: Shared,
    {
        DequeueAck {
            header: Header::new(
                Kind::DequeAck,
                self.header.version,
                self.header.uuid,
                value.len(),
            ),
            response: Response::success(),
            value: Some(value),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> DequeueAck<S> {
        DequeueAck {
            header: Header::new(Kind::DequeAck, self.header.version, self.header.uuid, 0),
            response: Response::fail(response_code, reason),
            value: None,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Dequeue<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Deque);

        let path = ByteStr::decode_owned(reader, buffer)?;

        Ok(Self { header, path })
    }
}

impl<W, S> Encode<W> for Dequeue<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;

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

    use super::Dequeue;

    #[test]
    fn acks() {
        let deque = Dequeue::new(1, 2, byte_str(b"test"));

        let ack = deque.clone().ack(binary_data(&[1, 2, 3]));
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = deque.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Dequeue(Dequeue::new(1, 2, byte_str(b"test"))));
    }
}
