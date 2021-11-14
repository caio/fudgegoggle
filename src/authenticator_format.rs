// Generated via `prost-build`, feeding it the proto schema from:
// https://raw.githubusercontent.com/beemdevelopment/Aegis/master/app/src/main/proto/google_auth.proto

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationPayload {
    #[prost(message, repeated, tag = "1")]
    pub otp_parameters: ::prost::alloc::vec::Vec<migration_payload::OtpParameters>,
    #[prost(int32, tag = "2")]
    pub version: i32,
    #[prost(int32, tag = "3")]
    pub batch_size: i32,
    #[prost(int32, tag = "4")]
    pub batch_index: i32,
    #[prost(int32, tag = "5")]
    pub batch_id: i32,
}
/// Nested message and enum types in `MigrationPayload`.
pub mod migration_payload {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OtpParameters {
        #[prost(bytes = "vec", tag = "1")]
        pub secret: ::prost::alloc::vec::Vec<u8>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub issuer: ::prost::alloc::string::String,
        #[prost(enumeration = "Algorithm", tag = "4")]
        pub algorithm: i32,
        #[prost(enumeration = "DigitCount", tag = "5")]
        pub digits: i32,
        #[prost(enumeration = "OtpType", tag = "6")]
        pub r#type: i32,
        #[prost(int64, tag = "7")]
        pub counter: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Algorithm {
        Unspecified = 0,
        Sha1 = 1,
        Sha256 = 2,
        Sha512 = 3,
        Md5 = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DigitCount {
        Unspecified = 0,
        Six = 1,
        Eight = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OtpType {
        Unspecified = 0,
        Hotp = 1,
        Totp = 2,
    }
}
