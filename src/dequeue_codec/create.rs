use std::io::{Read, Write};

use crate::{
    buffer::{ByteStr, Owned, Shared},
    header::{Uuid, Version},
    Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::CreateAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Create<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
    pub(crate) node_size: u64,
    pub(crate) max_disk_usage: u64,
}

impl<S> Create<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        path: ByteStr<S>,
        node_size: u64,
        max_disk_usage: u64,
    ) -> Self {
        Self {
            header: Header::new(Kind::CreateQueue, version, uuid, path.len()),
            path,
            node_size,
            max_disk_usage,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn node_size(&self) -> u64 {
        self.node_size
    }

    pub fn max_disk_usage(&self) -> u64 {
        self.max_disk_usage
    }

    pub fn ack(self) -> CreateAck {
        CreateAck {
            header: Header::new(
                Kind::CreateQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> CreateAck {
        CreateAck {
            header: Header::new(
                Kind::CreateQueueAck,
                self.header.version,
                self.header.uuid,
                0,
            ),
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Create<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::CreateQueue);

        let path = ByteStr::decode_owned(reader, buffer)?;
        let node_size = u64::decode(reader)?;
        let max_disk_usage = u64::decode(reader)?;

        Ok(Self {
            header,
            path,
            node_size,
            max_disk_usage,
        })
    }
}

impl<W, S> Encode<W> for Create<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.node_size.encode(writer)?;
        self.max_disk_usage.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::byte_str, tests::verify_encode_decode, Ack, Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Create;

    #[test]
    fn test_new() {
        let create = Create::new(0, 1, byte_str(b"test"), 1024, 1024 * 1024);

        assert_eq!(create.header().version, 0.into());
        assert_eq!(create.header().uuid, 1.into());
        assert_eq!(create.path().as_slice(), b"test");
        assert_eq!(create.node_size(), 1024);
    }

    #[test]
    fn test_acks() {
        let create = Create::new(0, 1, byte_str(b"test"), 1024, 1024 * 1024);

        let ack = create.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = create.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        verify_encode_decode(Packet::CreateQueue(Create::new(
            0,
            1,
            byte_str(b"test"),
            1024,
            1024 * 1024,
        )));
    }
}
