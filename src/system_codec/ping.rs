use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use crate::{
    buffer::Owned,
    header::{Uuid, Version},
    Encode, Error, Header, Kind, PartialDecode, Shared,
};

use super::PingAck;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Ping<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    _phantom: PhantomData<S>,
}

impl<S> Ping<S>
where
    S: Shared,
{
    pub fn new(version: impl Into<Version>, uuid: impl Into<Uuid>) -> Self {
        Self {
            header: Header::new(Kind::Ping, version, uuid, 0),
            _phantom: PhantomData,
        }
    }

    pub fn header(&self) -> Header {
        self.header
    }

    pub fn ack(self) -> PingAck<S> {
        PingAck {
            header: Header::new(Kind::PingAck, self.header.version, self.header.uuid, 0),
            _phantom: PhantomData,
        }
    }
}

impl<R, O> PartialDecode<R, O> for Ping<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, _reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::Ping);

        Ok(Self {
            header,
            _phantom: PhantomData,
        })
    }
}

impl<W, S> Encode<W> for Ping<S>
where
    S: Shared,
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::verify_encode_decode, Ack, Packet, SharedImpl, SUCCESS};

    use super::Ping;

    #[test]
    fn test_ack() {
        let ping = Ping::<SharedImpl>::new(1, 2);
        let ping_ack = ping.ack();

        assert_eq!(ping_ack.response().code(), SUCCESS);
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::Ping(Ping::new(1, 2)));
    }
}
