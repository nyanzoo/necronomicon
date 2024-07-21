use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, ByteStr, Owned, Shared},
    header::{Uuid, Version},
    Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, SUCCESS,
};

use super::TransferAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Transfer<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) path: ByteStr<S>,
    pub(crate) offset: u64,
    pub(crate) content: BinaryData<S>,
}

impl<S> Transfer<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        path: ByteStr<S>,
        offset: u64,
        content: BinaryData<S>,
    ) -> Self {
        Self {
            header: Header::new(Kind::Transfer, version, uuid, path.len() + content.len()),
            path,
            offset,
            content,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn path(&self) -> &ByteStr<S> {
        &self.path
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn content(&self) -> &BinaryData<S> {
        &self.content
    }

    pub fn ack(self) -> TransferAck {
        TransferAck {
            header: Header::new(Kind::TransferAck, self.header.version, self.header.uuid, 1),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> TransferAck {
        TransferAck {
            header: Header::new(Kind::TransferAck, self.header.version, self.header.uuid, 1),
            response_code,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Transfer<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Transfer);

        let path = ByteStr::decode_owned(reader, buffer)?;
        let offset = u64::decode(reader)?;
        let content = BinaryData::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            path,
            offset,
            content,
        })
    }
}

impl<S, W> Encode<W> for Transfer<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.path.encode(writer)?;
        self.offset.encode(writer)?;
        self.content.encode(writer)?;

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

    use super::Transfer;

    #[test]
    fn acks() {
        let transfer = Transfer::new(
            1,
            2,
            byte_str(b"/tmp/kitty"),
            0,
            binary_data(&[0x01, 0x02, 0x03, 0x04]),
        );

        let ack = transfer.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = transfer.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Transfer(Transfer::new(
            1,
            2,
            byte_str(b"/tmp/kitty"),
            42,
            binary_data(&[0x01, 0x02, 0x03, 0x04]),
        )));
    }
}
