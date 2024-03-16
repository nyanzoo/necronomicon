use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use crate::{buffer::Owned, Ack, Encode, Error, Header, Kind, PartialDecode, Response, Shared};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PingAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) _phantom: PhantomData<S>,
}

impl<R, O> PartialDecode<R, O> for PingAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, _reader: &mut R, _: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::PingAck);

        Ok(Self {
            header,
            _phantom: PhantomData,
        })
    }
}

impl<W, S> Encode<W> for PingAck<S>
where
    S: Shared,
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack<S> for PingAck<S>
where
    S: Shared,
{
    fn header(&self) -> &Header {
        &self.header
    }

    fn response(&self) -> Response<S> {
        Response::success()
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{tests::verify_encode_decode, Header, Kind, Packet, SharedImpl};

    use super::PingAck;

    impl PingAck<SharedImpl> {
        fn new() -> Self {
            Self {
                header: Header::new_test_ack(Kind::PingAck),
                _phantom: PhantomData,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::PingAck(PingAck::new()));
    }
}
