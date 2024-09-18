#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use std::fmt::format;

use cfx_addr::{self, cfx_addr_decode, Network};
use napi::Error;
#[napi]
fn encode(addr: String) {}

#[napi(object)]
pub struct HexAddress {
  pub hexAddress: String,
  pub netId: u32,
  pub r#type: String,
}

#[napi]
fn decode(base32_address: String) -> Result<HexAddress, napi::Error> {
  let decode_res = cfx_addr_decode(&base32_address);

  if decode_res.is_err() {
    return Err(Error::from_reason("decode error: failed to decode"));
  }

  let address = decode_res.unwrap();

  let net_id: u32 = match address.network {
    Network::Main => 1029,
    Network::Test => 1,
    Network::Id(id) => id as u32,
  };
  let hex_address = address.hex_address.unwrap();

  let addr_type = if hex_address.is_zero() {
    "null"
  } else {
    match address.parsed_address_bytes[0] & 0xf0 {
      0x00 => "builtin",
      0x80 => "contract",
      0x10 => "user",
      _ => "invalid",
    }
  };

  Ok(HexAddress {
    hexAddress: "0x".to_string() + &hex_address.to_string(),
    netId: net_id,
    r#type: addr_type.to_string(),
  })
}
