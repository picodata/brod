use crate::protocol::codecs::encode_as_array;
use std::default::Default;
use std::io::Read;
use std::io::Write;

use crate::protocol::codecs::{FromByte, ToByte};
use crate::protocol::header::{RequestHeader, ResponseHeader};

use crate::kafka_error::Result;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ApiVersionsRequest {}

impl FromByte for ApiVersionsRequest {
    type R = RequestHeader;
    fn decode<T: Read>(&mut self, buffer: &mut T) -> Result<()> {
        Ok(())
    }
}



struct ApiKeyVersions {
    api_key: i16,
    min_version: i16,
    max_version: i16,
}

pub struct ApiVersionsResponse {
    response_header: ResponseHeader,
    error_code: i16,
    api_versions: std::vec::Vec<ApiKeyVersions>,
}

impl ApiVersionsResponse {
    pub fn new(corr_id: i32, min: i16, max: i16) -> ApiVersionsResponse {
        return ApiVersionsResponse{
            response_header: ResponseHeader{
                correlation: corr_id,
            },
            error_code: 0,
            api_versions: vec![ApiKeyVersions{
                api_key: 18,
                min_version: 1,
                max_version: 1,
            }],
        }
    }
}

impl ToByte for ApiKeyVersions {
    fn encode<T: Write>(&self, buffer: &mut T) -> Result<()> {
        self.api_key.encode(buffer)?;
        self.min_version.encode(buffer)?;
        self.max_version.encode(buffer)?;
        Ok(())
    }
}

impl ToByte for ApiVersionsResponse {
    fn encode<T: Write>(&self, buffer: &mut T) -> Result<()> {
        self.response_header.encode(buffer)?;
        self.error_code.encode(buffer)?;
        self.api_versions.encode(buffer)?;
        Ok(())
    }
}
