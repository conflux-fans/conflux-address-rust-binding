use cfx_addr::*;
use cfx_addr::errors::EncodingError;
use neon::prelude::*;
use neon::result::Throw;

const MAIN: u32 = 1029;
const TEST: u32 = 1;

/*
TODO
1. Use buffer to receive the first argument
2. Add unit test and benchmark
*/
// NOTE: only accept lowercase hex address
fn encode(mut cx: FunctionContext) -> JsResult<JsString> {
    let hex_handle = cx.argument::<JsString>(0)?;
    let net_id_handle = cx.argument::<JsNumber>(1)?;
    let verbose_handle = cx.argument_opt(2);

    // prepare hex address
    let hex_address: String = hex_handle.value(&mut cx);
    let hex_str = hex_address.as_str().trim_start_matches("0x");
    let raw = hex::decode(hex_str).map_err(|_err| Throw)?;
    // prepare networkId
    let net_id: u32 = net_id_handle.value(&mut cx) as u32;
    let network: Network = match net_id {
        TEST => Network::Test,
        MAIN => Network::Main,
        _ => Network::Id(net_id as u64),
    };
    // prepare option
    let verbose = if let Some(v) = verbose_handle {
        if !v.is_a::<JsBoolean, FunctionContext>(&mut cx) {
            let _e = cx.throw_error::<&str, JsError>("The 3rd argument (verbose) should be a boolean");
            return Err(Throw);
        };
        v.downcast::<JsBoolean, FunctionContext>(&mut cx)
            .map_err(|_err| Throw)?
            .value(&mut cx) as bool
    } else {
        false
    };
    let option = if verbose {
        EncodingOptions::QrCode
    } else {
        EncodingOptions::Simple
    };

    match cfx_addr_encode(&raw, network, option) {
        Ok(base32_address) => Ok(cx.string(base32_address.as_str())),
        Err(err) => {
            let error_msg = match err {
                EncodingError::InvalidAddressType(t) => format!("Invalid address type: {}", t),
                EncodingError::InvalidLength(l) => format!("Invalid length: {}", l),
                EncodingError::InvalidNetworkId(net_id) => format!("Invalid netId: {}", net_id),
            };
            let _e = cx.throw_error::<&str, JsError>(error_msg.as_str());
            Err(Throw)
        }
    }
}

fn decode(mut cx: FunctionContext) -> JsResult<JsObject> {
    let base32: String = cx.argument::<JsString>(0)?.value(&mut cx);
    // decode and check result
    let decode_res = cfx_addr_decode(base32.as_str());
    if let Err(err) = decode_res {
        let error_msg = match err {
            DecodingError::InvalidLength(_) => "InvalidLength",
            DecodingError::NoPrefix => "NoPrefix",
            DecodingError::InvalidPrefix(_) => "InvalidPrefix",
            DecodingError::InvalidOption(_) => "InvalidOption",
            DecodingError::ChecksumFailed(_) => "ChecksumFailed",
            DecodingError::InvalidChar(_) => "InvalidChar",
            DecodingError::InvalidPadding { .. } => "InvalidPadding",
            DecodingError::VersionNotRecognized(_) => "VersionNotRecognized",
            DecodingError::MixedCase => "MixedCase",
        };
        let _e = cx.throw_error::<&str, JsError>(error_msg);
        return Err(Throw);
    }
    let decoded = decode_res.unwrap();
    if decoded.hex_address.is_none() {
        let _e = cx.throw_error::<&str, JsError>("Decode failed");
        return Err(Throw);
    }

    let net_id = match decoded.network {
        Network::Main => MAIN,
        Network::Test => TEST,
        Network::Id(id) => id as u32,
    };

    let address_type = if decoded.hex_address.unwrap().is_zero() {
        "null"
    } else {
        match decoded.parsed_address_bytes[0] & 0xf0 {
            0x00 => "builtin",
            0x80 => "contract",
            0x10 => "user",
            _ => "invalid",
        }
    };

    let obj = cx.empty_object();
    let hex_address_handle = JsBuffer::external(&mut cx, decoded.parsed_address_bytes);
    let net_id_handle = cx.number(net_id);
    let address_type_handle = cx.string(address_type);
    obj.set(&mut cx, "hexAddress", hex_address_handle)?;
    obj.set(&mut cx, "netId", net_id_handle)?;
    obj.set(&mut cx, "type", address_type_handle)?;

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("encode", encode)?;
    cx.export_function("decode", decode)?;
    Ok(())
}
