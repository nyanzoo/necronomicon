use std::io::{Read, Write};

use crate::{
    buffer::Owned, Ack, Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
    Shared,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct LenAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
    pub(crate) len: u64,
}

impl<R, O> PartialDecode<R, O> for LenAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::LenAck);

        let response = Response::decode_owned(reader, buffer)?;
        let len = u64::decode(reader)?;

        Ok(Self {
            header,
            response,
            len,
        })
    }
}

impl<W, S> Encode<W> for LenAck<S>
where
    S: Shared,
    W: Write,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response.encode(writer)?;
        self.len.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack<S> for LenAck<S>
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

    use super::LenAck;

    impl LenAck<SharedImpl> {
        pub fn new(response: Response<SharedImpl>, len: u64) -> Self {
            Self {
                header: Header::new(Kind::LenAck, 1, 1, 0),
                response,
                len,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::LenAck(LenAck::new(Response::success(), 123)));
    }
}
