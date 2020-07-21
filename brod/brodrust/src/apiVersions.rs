pub struct ApiVersionsRequest {
}

pub struct ApiVersionsResponse {
    error_code: Option<u8>;
    min_version: i16;
    max_version: i16;
}