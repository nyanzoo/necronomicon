use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::EnqueueAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Enqueue<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
    pub(crate) value: BinaryData<S>,
}

impl<S> Enqueue<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        path: ByteStr<S>,
        value: BinaryData<S>,
    ) -> Self {
        Self {
            header: Header::new(Kind::Enqueue, version, uuid, path.len() + value.len()),
            path,
            value,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn value(&self) -> &BinaryData<S> {
        &self.value
    }

    pub fn ack(self) -> EnqueueAck {
        EnqueueAck {
            header: Header::new(Kind::EnqueueAck, self.header.version, self.header.uuid, 0),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> EnqueueAck {
        EnqueueAck {
            header: Header::new(Kind::EnqueueAck, self.header.version, self.header.uuid, 0),
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Enqueue<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Enqueue);

        let path = ByteStr::decode_owned(reader, buffer)?;
        let value = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            path,
            value,
        })
    }
}

impl<W, S> Encode<W> for Enqueue<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.value.encode(writer)?;

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

    use super::Enqueue;

    #[test]
    fn acks() {
        let enqueue = Enqueue::new(0, 0, byte_str(b"test"), binary_data(&[1, 2, 3]));

        let ack = enqueue.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = enqueue.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Enqueue(Enqueue::new(
            1,
            1,
            byte_str(b"test"),
            binary_data(&[1, 2, 3]),
        )));
    }
}
