use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, Owned, Shared},
    header::{Uuid, Version},
    DecodeOwned, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::GetAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Get<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) key: BinaryData<S>,
}

impl<S> Get<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>, key: BinaryData<S>) -> Self {
        Self {
            header: Header::new(Kind::Get, version, uuid, key.len()),
            key,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn key(&self) -> &BinaryData<S> {
        &self.key
    }

    pub fn ack(self, value: BinaryData<S>) -> GetAck<S> {
        GetAck {
            header: Header::new(
                Kind::GetAck,
                self.header.version,
                self.header.uuid,
                value.len(),
            ),
            response_code: SUCCESS,
            value: Some(value),
        }
    }

    pub fn nack(self, response_code: u8) -> GetAck<S> {
        GetAck {
            header: Header::new(Kind::GetAck, self.header.version, self.header.uuid, 0),
            response_code,
            value: None,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Get<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Get);

        let key = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self { header, key })
    }
}

impl<W, S> Encode<W> for Get<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.key.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::binary_data, kv_store_codec::test_key, tests::verify_encode_decode, Ack, Kind,
        Packet, INTERNAL_ERROR, SUCCESS,
    };

    use super::Get;

    #[test]
    fn test_new() {
        let get = Get::new(1, 1, test_key());

        assert_eq!(get.header().kind, Kind::Get);
        assert_eq!(get.header().version, 1.into());
        assert_eq!(get.header().uuid, 1.into());
        assert_eq!(get.key(), &test_key());
    }

    #[test]
    fn acks() {
        let get = Get::new(1, 1, test_key());

        let ack = get.clone().ack(binary_data(&[1, 2, 3]));
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = get.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Get(Get::new(1, 1, test_key())));
    }
}
