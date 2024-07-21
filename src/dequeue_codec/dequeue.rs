use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
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
            header: Header::new(Kind::Dequeue, version, uuid, path.len()),
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
                Kind::DequeueAck,
                self.header.version,
                self.header.uuid,
                value.len(),
            ),
            response_code: SUCCESS,
            value: Some(value),
        }
    }

    pub fn nack(self, response_code: u8) -> DequeueAck<S> {
        DequeueAck {
            header: Header::new(Kind::DequeueAck, self.header.version, self.header.uuid, 0),
            response_code,
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
        assert_eq!(header.kind, Kind::Dequeue);

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
        let dequeue = Dequeue::new(1, 2, byte_str(b"test"));

        let ack = dequeue.clone().ack(binary_data(&[1, 2, 3]));
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = dequeue.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Dequeue(Dequeue::new(1, 2, byte_str(b"test"))));
    }
}
