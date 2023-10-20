use std::io::{Read, Write};

use crate::{header::VersionAndUuid, Decode, Encode, Error, Header, Kind, PartialDecode, SUCCESS};

use super::{JoinAck, Role};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Join {
    pub(crate) header: Header,
    pub(crate) role: Role,
    pub(crate) version: u128,
    pub(crate) successor_lost: bool,
}

impl Join {
    pub fn new(
        version_and_uuid: impl Into<VersionAndUuid>,
        role: Role,
        version: u128,
        successor_lost: bool,
    ) -> Self {
        Self {
            header: version_and_uuid.into().into_header(Kind::Join),
            role,
            version,
            successor_lost,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn store_version(&self) -> u128 {
        self.version
    }

    pub fn successor_lost(&self) -> bool {
        self.successor_lost
    }

    pub fn addr(&self) -> Option<&str> {
        match &self.role {
            Role::Backend(addr) => Some(addr),
            Role::Frontend(addr) => Some(addr),
            Role::Observer => None,
        }
    }

    pub fn ack(self) -> JoinAck {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version(), self.header.uuid()),
            response_code: SUCCESS,
        }
    }

    pub fn nack(self, response_code: u8) -> JoinAck {
        JoinAck {
            header: Header::new(Kind::JoinAck, self.header.version(), self.header.uuid()),
            response_code,
        }
    }
}

impl<R> PartialDecode<R> for Join
where
    R: Read,
{
    fn decode(header: Header, reader: &mut R) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind(), Kind::Join);

        let role = Role::decode(reader)?;
        let version = u128::decode(reader)?;
        let successor_lost = u8::decode(reader)? > 0;

        Ok(Self {
            header,
            role,
            version,
            successor_lost,
        })
    }
}

impl<W> Encode<W> for Join
where
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.role.encode(writer)?;
        self.version.encode(writer)?;
        u8::from(self.successor_lost).encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        system_codec::Role, tests::test_encode_decode_packet, Ack, Header, Kind, INTERNAL_ERROR,
        SUCCESS,
    };

    use super::Join;

    #[test]
    fn test_getters() {
        let join = Join::new((1, 2), Role::Backend("localhost".to_string()), 1, false);

        assert_eq!(join.header(), Header::new(Kind::Join, 1, 2));
        assert_eq!(join.role(), &Role::Backend("localhost".to_string()));
        assert_eq!(join.store_version(), 1);
        assert_eq!(join.successor_lost(), false);
        assert_eq!(join.addr(), Some("localhost"));
    }

    #[test]
    fn test_acks() {
        let join = Join::new((1, 2), Role::Backend("localhost".to_string()), 1, false);

        let ack = join.clone().ack();
        assert_eq!(ack.response_code(), SUCCESS);

        let nack = join.nack(INTERNAL_ERROR);
        assert_eq!(nack.response_code(), INTERNAL_ERROR);
    }

    #[test]
    fn test_encode_decode() {
        test_encode_decode_packet!(
            Kind::Join,
            Join {
                role: Role::Backend("localhost".to_string()),
                version: 1,
                successor_lost: false,
            }
        );
    }
}
