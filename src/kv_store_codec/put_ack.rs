use std::io::{Read, Write};

use crate::{
    buffer::Owned, Ack, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response, Shared,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PutAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
}

impl<R, O> PartialDecode<R, O> for PutAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::PutAck);

        let response = Response::decode_owned(reader, buffer)?;

        Ok(Self { header, response })
    }
}

impl<W, S> Encode<W> for PutAck<S>
where
    S: Shared,
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack<S> for PutAck<S>
where
    S: Shared,
{
    fn header(&self) -> &Header {
        &self.header
    }

    fn response(&self) -> Response<S> {
        self.response.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::verify_encode_decode, Header, Kind, Packet, Response, SharedImpl};

    use super::PutAck;

    impl PutAck<SharedImpl> {
        pub fn new(response: Response<SharedImpl>) -> Self {
            Self {
                header: Header::new(Kind::PutAck, 1, 1, 0),
                response,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::PutAck(PutAck::new(Response::success())));
    }
}
