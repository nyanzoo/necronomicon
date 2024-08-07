use std::io::{Read, Write};

use crate::{
    buffer::{ByteStr, Owned, Shared},
    header::{Uuid, Version},
    Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
};

use super::{JoinAck, Role};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Join<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) role: Role<S>,
    pub(crate) instance: u128,
    pub(crate) successor_lost: bool,
}

impl<S> Join<S>
where
    S: Shared,
{
    pub fn new(
        version: impl Into<Version>,
        uuid: impl Into<Uuid>,
        role: Role<S>,
        instance: u128,
        successor_lost: bool,
    ) -> Self {
        Self {
            header: Header::new(Kind::Join, version, uuid, role.encode_len()),
            role,
            instance,
            successor_lost,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn role(&self) -> &Role<S> {
        &self.role
    }

    pub fn store_version(&self) -> u128 {
        self.instance
    }

    pub fn successor_lost(&self) -> bool {
        self.successor_lost
    }

    pub fn addr(&self) -> Option<&ByteStr<S>> {
        match &self.role {
            Role::Backend(addr) => Some(addr),
            Role::Frontend(addr) => Some(addr),
            Role::Observer => None,
        }
    }

    pub fn ack(self) -> JoinAck<S> {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version, self.header.uuid, 0),
            response: Response::success(),
        }
    }

    pub fn nack(self, response_code: u8, reason: Option<ByteStr<S>>) -> JoinAck<S> {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version, self.header.uuid, 0),
            response: Response::fail(response_code, reason),
        }
    }
}

impl<R, O> PartialDecode<R, O> for Join<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Join);

        let role = Role::decode_owned(reader, buffer)?;
        let version = u128::decode(reader)?;
        let successor_lost = u8::decode(reader)? > 0;

        Ok(Self {
            header,
            role,
            instance: version,
            successor_lost,
        })
    }
}

impl<W, S> Encode<W> for Join<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.role.encode(writer)?;
        self.instance.encode(writer)?;
        u8::from(self.successor_lost).encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::byte_str, system_codec::Role, tests::verify_encode_decode, Ack, Packet,
        INTERNAL_ERROR, SUCCESS,
    };

    use super::Join;

    #[test]
    fn acks() {
        let join = Join::new(1, 2, Role::Backend(byte_str(b"localhost")), 1, false);

        let ack = join.clone().ack();
        assert_eq!(ack.response().code(), SUCCESS);

        let nack = join.nack(INTERNAL_ERROR, None);
        assert_eq!(nack.response().code(), INTERNAL_ERROR);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Join(Join::new(
            1,
            1,
            Role::Backend(byte_str(b"localhost")),
            1,
            false,
        )));
    }
}
