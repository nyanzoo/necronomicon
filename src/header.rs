use std::{
    io::{Read, Write},
    mem::size_of,
};

use crate::{dequeue_codec, error::Error, kv_store_codec, Encode};

// TODO: need to map these to packet types, also need to do partial
// decodes of header to get packet type and then decode the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    // dequeue messages
    // make sure to keep these in sync with the ones in
    // necronomicon/src/dequeue_codec/mod.rs
    Enqueue = dequeue_codec::START as isize,
    EnqueueAck,
    Dequeue,
    DequeueAck,
    Peek,
    PeekAck,
    Len,
    LenAck = dequeue_codec::END as isize,

    // kv store messages
    // make sure to keep these in sync with the ones in
    // necronomicon/src/kv_store_codec/mod.rs
    Put = kv_store_codec::START as isize,
    PutAck,
    Get,
    GetAck,
    Delete,
    DeleteAck = kv_store_codec::END as isize,

    // internal system messages
    Patch = 0xff,
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            // dequeue messages
            dequeue_codec::ENQUEUE => Kind::Enqueue,
            dequeue_codec::ENQUEUE_ACK => Kind::EnqueueAck,
            dequeue_codec::DEQUEUE => Kind::Dequeue,
            dequeue_codec::DEQUEUE_ACK => Kind::DequeueAck,
            dequeue_codec::PEEK => Kind::Peek,
            dequeue_codec::PEEK_ACK => Kind::PeekAck,
            dequeue_codec::LEN => Kind::Len,
            dequeue_codec::LEN_ACK => Kind::LenAck,

            // kv store messages
            kv_store_codec::PUT => Kind::Put,
            kv_store_codec::PUT_ACK => Kind::PutAck,
            kv_store_codec::GET => Kind::Get,
            kv_store_codec::GET_ACK => Kind::GetAck,
            kv_store_codec::DELETE => Kind::Delete,
            kv_store_codec::DELETE_ACK => Kind::DeleteAck,

            // internal system messages
            0xff => Kind::Patch,

            _ => panic!("invalid kind: {}", value),
        }
    }
}

impl From<Kind> for u8 {
    fn from(value: Kind) -> Self {
        match value {
            // dequeue messages
            Kind::Enqueue => dequeue_codec::ENQUEUE,
            Kind::EnqueueAck => dequeue_codec::ENQUEUE_ACK,
            Kind::Dequeue => dequeue_codec::DEQUEUE,
            Kind::DequeueAck => dequeue_codec::DEQUEUE_ACK,
            Kind::Peek => dequeue_codec::PEEK,
            Kind::PeekAck => dequeue_codec::PEEK_ACK,
            Kind::Len => dequeue_codec::LEN,
            Kind::LenAck => dequeue_codec::LEN_ACK,

            // kv store messages
            Kind::Put => kv_store_codec::PUT,
            Kind::PutAck => kv_store_codec::PUT_ACK,
            Kind::Get => kv_store_codec::GET,
            Kind::GetAck => kv_store_codec::GET_ACK,
            Kind::Delete => kv_store_codec::DELETE,
            Kind::DeleteAck => kv_store_codec::DELETE_ACK,

            // internal system messages
            Kind::Patch => 0xff,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Header {
    kind: u8,
    version: u8,
    uuid: u128,
}

impl Header {
    pub fn new(kind: Kind, version: u8, uuid: u128) -> Self {
        Self {
            kind: kind.into(),
            version,
            uuid,
        }
    }

    pub fn kind(&self) -> Kind {
        self.kind.into()
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn uuid(&self) -> u128 {
        self.uuid
    }

    pub fn decode(reader: &mut impl Read) -> Result<Self, Error> {
        let mut bytes = [0; size_of::<Header>()];
        reader
            .read_exact(&mut bytes)
            .map_err(Error::IncompleteHeader)?;

        let mut header = Header {
            kind: 0,
            version: 0,
            uuid: 0,
        };

        // Todo: verify kind
        header.kind = bytes[0];
        // Todo: verify version
        header.version = bytes[1];
        // Todo: verify length?
        header.uuid = u128::from_be_bytes(bytes[2..18].try_into().expect("not u64"));

        Ok(header)
    }
}

impl Encode for Header {
    fn encode(&self, writer: &mut impl Write) -> Result<(), Error> {
        let mut buf = [0; size_of::<Header>()];
        buf[0] = self.kind;
        buf[1] = self.version;
        buf[2..18].copy_from_slice(&self.uuid.to_be_bytes());

        writer.write_all(&buf).map_err(Error::Encode)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use test_case::test_case;

    use crate::Encode;

    use super::Header;

    #[test_case(0, 0, 0; "zero")]
    #[test_case(1, 1, 1; "one")]
    #[test_case(2, 2, 2; "two")]
    fn test_header_encode_decode(kind: u8, version: u8, uuid: u128) {
        let mut buf = Vec::new();
        let header = Header {
            kind,
            version,
            uuid,
        };
        header.encode(&mut buf).expect("encode");

        let mut buf = Cursor::new(buf);
        let actual = Header::decode(&mut buf).expect("decode");
        assert_eq!(actual, header);
    }
}
