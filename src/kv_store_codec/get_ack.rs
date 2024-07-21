use std::io::{Read, Write};

use crate::{
    buffer::{BinaryData, Owned, Shared},
    Ack, Decode, DecodeOwned, Encode, Error, Header, Kind, PartialDecode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAck<S>
where
    S: Shared,
{
    pub(crate) header: Header,
    pub(crate) response_code: u8,
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

        let response_code = u8::decode(reader)?;
        let value = Option::decode_owned(reader, buffer)?;

        Ok(Self {
            header,
            response_code,
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
        self.response_code.encode(writer)?;
        self.value.encode(writer)?;

        Ok(())
    }
}

impl<S> Ack for GetAck<S>
where
    S: Shared,
{
    fn header(&self) -> &Header {
        &self.header
    }

    fn response_code(&self) -> u8 {
        self.response_code
    }
}

#[cfg(test)]
mod test {
    use crate::{
        buffer::{BinaryData, Shared},
        tests::verify_encode_decode,
        Header, Kind, Packet, SUCCESS,
    };

    use super::GetAck;

    impl<S> GetAck<S>
    where
        S: Shared,
    {
        pub fn new(response_code: u8, value: Option<BinaryData<S>>) -> Self {
            Self {
                header: Header::new(Kind::GetAck, 1, 1, 0),
                response_code,
                value,
            }
        }
    }

    #[test]
    fn encode_decode() {
        verify_encode_decode(Packet::GetAck(GetAck::new(SUCCESS, None)));
    }
}
