use std::io::Write;
use std::io::Read;

use crate::protocol::codecs::{FromByte, ToByte};
use crate::kafka_error::{Result};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
}

impl RequestHeader {
    pub fn new<T: Read>(buffer: &mut T) -> RequestHeader {
        let mut hr = RequestHeader {
            api_key: 0,
            api_version: 0,
            correlation_id: 0,
            client_id: "".to_string(),
        };

        match hr.decode(buffer) {
            Err(e) => panic!(e),
            _ => (),
        };

        hr
    }
}

impl FromByte for RequestHeader {
    type R = RequestHeader;

    fn decode<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        self.api_key.decode(buffer)?;
        self.api_version.decode(buffer)?;
        self.correlation_id.decode(buffer)?;
        self.client_id.decode(buffer)?;
        Ok(())
    }
}

impl ToByte for RequestHeader {
    fn encode<T: Write>(&self, buffer: &mut T) -> Result<()> {
        self.api_key.encode(buffer)?;
        self.api_version.encode(buffer)?;
        self.correlation_id.encode(buffer)?;
        self.client_id.encode(buffer)?;
        Ok(())
    }
}

#[test]
fn to_and_from_byte() {
    use std::io::Cursor;

    let orig: RequestHeader = RequestHeader{
        api_key: 1,
        api_version: 2,
        correlation_id: 3,
        client_id: "client_id".to_string(),
    };

    let mut buf = vec![];
    // Encode into buffer
    orig.encode(&mut buf).unwrap();
    print!("{:?}", buf);
    assert_eq!(buf, [0, 1, 0, 2, 0, 0, 0, 3, 0, 9, 99, 108, 105, 101, 110, 116, 95, 105, 100]);

    // Read from buffer into existing variable
    let mut got: RequestHeader = Default::default();

    got.decode(&mut Cursor::new(&buf[..])).unwrap();
    assert_eq!(orig, got);
}