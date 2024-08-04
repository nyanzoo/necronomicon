use std::io::{Read, Write};

use crate::{
    buffer::Owned, Ack, DecodeOwned, Encode, Error, Header, Kind, PartialDecode, Response, Shared,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CreateAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response: Response<S>,
}

impl<R, O> PartialDecode<R, O> for CreateAck<O::Shared>
where
    R: Read,
    O: Owned,
{
    fn decode(header: Header, reader: &mut R, buffer: &mut O) -> Result<Self, Error>
    where
        Self: Sized,
    {
        assert_eq!(header.kind, Kind::CreateQueueAck);

        let response = Response::decode_owned(reader, buffer)?;

        Ok(Self { header, response })
    }
}

impl<W, S> Encode<W> for CreateAck<S>
where
    W: Write,
    S: Shared,
{
    fn encode(&self, writer: &mut W) -> Result<(), Error> {
        self.header.encode(writer)?;
        self.response.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack<S> for CreateAck<S>
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
        tests::verify_encode_decode, BinaryData, ByteStr, Header, Kind, Packet, Pool, PoolImpl,
        Response, SharedImpl, QUEUE_ALREADY_EXISTS,
    };

    use super::CreateAck;

    impl CreateAck<SharedImpl> {
        pub fn new(response: Response<SharedImpl>) -> Self {
            Self {
                header: Header::new_test_ack(Kind::CreateQueueAck),
                response,
            }
        }
    }

    #[test]
    fn encode_decode() {
        let pool = PoolImpl::new(1024, 1024);
        let mut buffer = pool.acquire("test");
        let value = ByteStr::new(BinaryData::from_owned("kittens", &mut buffer).expect("data"));
        verify_encode_decode(Packet::CreateQueueAck(CreateAck::new(Response::fail(
            QUEUE_ALREADY_EXISTS,
            Some(value),
        ))));

        verify_encode_decode(Packet::CreateQueueAck(CreateAck::new(Response::success())));
    }
}
