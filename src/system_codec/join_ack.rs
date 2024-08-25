use std::io::{Read, Write};

use crate::{
    buffer::Owned, Ack, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response, Shared,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct JoinAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
}

impl<R, O> PartialDecode<R, O> for JoinAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::JoinAck);

        let response = Response::decode_owned(reader, buffer)?;

        Ok(Self { header, response })
    }
}

impl<W, S> Encode<W> for JoinAck<S>
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

impl<S> Ack<S> for JoinAck<S>
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

#[cfg(any(test, feature = "test"))]
impl<S> JoinAck<S>
where
    S: Shared,
{
    pub fn new(response: Response<S>, uuid: u128) -> Self {
        Self {
            header: Header::new_test_full(Kind::JoinAck, 0, uuid),
            response,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{tests::verify_encode_decode, Response, SystemPacket};

    use super::JoinAck;

    #[test]
    fn encode_decode() {
        verify_encode_decode(SystemPacket::JoinAck(JoinAck::new(Response::success(), 1)));
    }
}
