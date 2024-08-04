use std::io::{Read, Write};

use log::trace;

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
};

use super::PutAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Put<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) key: BinaryData<S>,
    pub(crate) value: BinaryData<S>,
}

impl<S> Put<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        key: BinaryData<S>,
        value: BinaryData<S>,
    ) -> Self {
        Self {
            header: Header::new(Kind::Put, version, uuid, key.len() + value.len()),
            key,
            value,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn key(&self) -> &BinaryData<S> {
        &self.key
    }

    pub fn value(&self) -> &BinaryData<S> {
        &self.value
    }

    pub fn ack(self) -> PutAck<S> {
        PutAck {
            header: Header::new(Kind::PutAck, self.header.version, self.header.uuid, 0),
            response: Response::success(),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> PutAck<S> {
        PutAck {
            header: Header::new(Kind::PutAck, self.header.version, self.header.uuid, 0),
            response: Response::fail(response_code, reason),
        }
    }
}

impl<R, O> PartialDecode<R, O> for Put<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Put);

        trace!("decode put key");
        let key = BinaryData::decode_owned(reader, buffer)?;
        trace!("decode put value");
        let value = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self { header, key, value })
    }
}

impl<W, S> Encode<W> for Put<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.key.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::binary_data,
        codes::{INTERNAL_ERROR, SUCCESS},
        kv_store_codec::test_key,
        tests::verify_encode_decode,
        Ack, Packet,
    };

    use super::Put;

    #[test]
    fn test_ack() {
        let put = Put::new(0, 0, test_key(), binary_data(&[1, 2, 3]));

        let ack = put.clone().ack();
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = put.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Put(Put::new(
            1,
            1,
            test_key(),
            binary_data(&[1, 2, 3]),
        )));
    }
}
