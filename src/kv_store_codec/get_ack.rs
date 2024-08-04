use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, Owned, Shared},
    Ack, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
    pub(crate) value: Option<BinaryData<S>>,
}

impl<S> GetAck<S>
where
    S: Shared,
{
    pub fn value(&self) -> Option<&BinaryData<S>> {
        self.value.as_ref()
    }
}

impl<R, O> PartialDecode<R, O> for GetAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::GetAck);

        let response = Response::decode_owned(reader, buffer)?;
        let value = Option::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            response,
            value,
        })
    }
}

impl<W, S> Encode<W> for GetAck<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack<S> for GetAck<S>
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
    use crate::{
        buffer::BinaryData, tests::verify_encode_decode, Header, Kind, Packet, Response, SharedImpl,
    };

    use super::GetAck;

    impl GetAck<SharedImpl> {
        pub fn new(response: Response<SharedImpl>, value: Option<BinaryData<SharedImpl>>) -> Self {
            Self {
                header: Header::new(Kind::GetAck, 1, 1, 0),
                response,
                value,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::GetAck(GetAck::new(Response::success(), None)));
    }
}
