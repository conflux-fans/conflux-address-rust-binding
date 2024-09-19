#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use cfx_addr::{self, cfx_addr_decode, errors::OptionError, DecodingError, Network};
use napi::Error;

#[napi]
pub const MAIN_NET_ID: u32 = 1029;
#[napi]
pub const TEST_NET_ID: u32 = 1;

#[napi]
fn encode(addr: String) {}

#[napi(object)]
pub struct DecodeResult {
  pub hexAddress: String,
  pub netId: u32,
  pub r#type: String,
}

#[napi]
fn decode(base32_address: String) -> Result<DecodeResult, napi::Error> {
  let decode_res = cfx_addr_decode(&base32_address);

  match decode_res {
    Ok(decode_raw_address) => {
      let net_id: u32 = match decode_raw_address.network {
        Network::Main => MAIN_NET_ID,
        Network::Test => TEST_NET_ID,
        Network::Id(id) => id as u32,
      };

      let hex_address = decode_raw_address.hex_address.unwrap();

      let address_type = if hex_address.is_zero() {
        "null"
      } else {
        match decode_raw_address.parsed_address_bytes[0] & 0xf0 {
          0x00 => "builtin",
          0x80 => "contract",
          0x10 => "user",
          _ => "invalid",
        }
      };

      Ok(DecodeResult {
        hexAddress: format!("0x{}", hex_address),
        netId: net_id,
        r#type: address_type.to_string(),
      })
    }
    Err(err) => {
      let error_msg = match err {
        DecodingError::ChecksumFailed { .. } => "invalid checksum",
        DecodingError::InvalidChar(_) => "invalid char",
        DecodingError::InvalidLength(_) => "invalid length",
        DecodingError::InvalidOption(option_error) => match option_error {
          OptionError::AddressTypeMismatch { .. } => {
            "decoded address does not match address type in option"
          }
          OptionError::ParseError(_) => "invalid option",
          OptionError::InvalidAddressType(_) => "invalid address type",
        },
        DecodingError::InvalidPadding { .. } => "invalid padding",
        DecodingError::InvalidPrefix(_) => "invalid prefix",
        DecodingError::NoPrefix => "zero or multiple prefixes",
        DecodingError::MixedCase => "mixed case string",
        DecodingError::VersionNotRecognized(_) => "version byte not recognized",
      };

      Err(Error::from_reason(format!("decode error: {}", error_msg)))
    }
  }
}
