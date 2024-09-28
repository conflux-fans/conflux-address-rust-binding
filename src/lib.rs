#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use cfx_addr::{
  self, cfx_addr_decode, cfx_addr_encode,
  errors::{EncodingError, OptionError},
  DecodingError, EncodingOptions, Network,
};
use napi::{bindgen_prelude::Buffer, Error};

#[napi]
pub const MAIN_NET_ID: u32 = 1029;
#[napi]
pub const TEST_NET_ID: u32 = 1;

#[napi]
fn encode(hex_address: String, net_id: u32, verbose: bool) -> Result<String, napi::Error> {
  let network_id = match net_id {
    MAIN_NET_ID => Network::Main,
    TEST_NET_ID => Network::Test,
    _ => Network::Id(net_id as u64),
  };

  let hex_str = hex_address.as_str().trim_start_matches("0x");

  let encoding_options = if verbose {
    EncodingOptions::QrCode
  } else {
    EncodingOptions::Simple
  };

  let raw_addr =
    hex::decode(hex_str).map_err(|_| Error::from_reason("Error: encode error: invalid hex string"))?;

  let base32_address =
    cfx_addr_encode(&raw_addr, network_id, encoding_options).map_err(|e| match e {
      EncodingError::InvalidLength(_) => Error::from_reason("Error: encode error: invalid length"),
      EncodingError::InvalidAddressType(_) => {
        Error::from_reason("Error: encode error: invalid address type")
      }
      EncodingError::InvalidNetworkId(id) => {
        Error::from_reason(format!("Error: encode error: invalid network id: {}", id))
      }
    })?;

  Ok(base32_address)
}

#[napi(object)]
pub struct DecodeResult {
  pub hexAddress: Buffer,
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
        hexAddress: decode_raw_address.parsed_address_bytes.into(),
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

      Err(Error::from_reason(format!("Error: decode error {}", error_msg)))
    }
  }
}
