use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, Owned, Shared},
    Ack, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DequeueAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
    pub(crate) value: Option<BinaryData<S>>,
}

impl<R, O> PartialDecode<R, O> for DequeueAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::DequeAck);

        let response = Response::decode_owned(reader, buffer)?;
        let value = Option::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            response,
            value,
        })
    }
}

impl<W, S> Encode<W> for DequeueAck<S>
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

impl<S> Ack<S> for DequeueAck<S>
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
mod tests {
    use crate::{
        buffer::{BinaryData, SharedImpl},
        response::Response,
        tests::verify_encode_decode,
        Header, Kind, Packet,
    };

    use super::DequeueAck;

    impl DequeueAck<SharedImpl> {
        pub fn new(response: Response<SharedImpl>, value: Option<BinaryData<SharedImpl>>) -> Self {
            Self {
                header: Header::new_test_ack(Kind::DequeAck),
                response,
                value,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::DequeueAck(DequeueAck::new(
            Response::success(),
            None,
        )));
    }
}
