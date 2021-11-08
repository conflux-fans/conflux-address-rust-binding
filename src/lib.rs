use cfx_addr::*;
use neon::prelude::*;
use neon::result::Throw;

const MAIN: u32 = 1029;
const TEST: u32 = 1;

// NOTE: only accept lowercase hex address
fn encode(mut cx: FunctionContext) -> JsResult<JsString> {
    let hex_handle = cx.argument::<JsString>(0)?; // TODO use buffer to receive the first argument
    let net_id_handle = cx.argument::<JsNumber>(1)?;
    let verbose_handle = cx.argument::<JsBoolean>(2).unwrap_or(cx.boolean(false));

    let hex_address: String = hex_handle.value(&mut cx) as String;
    let hex_str = hex_address.as_str().trim_start_matches("0x"); // TODO convert to lowercase before trimming
    let raw = hex::decode(hex_str).map_err(|_err| Throw)?;

    let net_id: u32 = net_id_handle.value(&mut cx) as u32;
    let network: Network = match net_id {
        TEST => Network::Test,
        MAIN => Network::Main,
        _ => Network::Id(net_id as u64),
    };

    // option
    let verbose: bool = verbose_handle.value(&mut cx) as bool;
    let option = if verbose {
        EncodingOptions::QrCode
    } else {
        EncodingOptions::Simple
    };

    let base32_address = cfx_addr_encode(&raw, network, option).map_err(|_err| Throw)?;
    Ok(cx.string(base32_address.as_str()))
}

fn decode(mut cx: FunctionContext) -> JsResult<JsObject> {
    let base32_handle = cx.argument::<JsString>(0)?;
    let base32: String = base32_handle.value(&mut cx) as String;

    let decoded = cfx_addr_decode(base32.as_str()).map_err(|_err| Throw)?;
    if decoded.hex_address.is_none() {
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

    let hex_address = format!("0x{}", hex::encode(decoded.parsed_address_bytes));

    let obj = cx.empty_object();
    let hex_address_handle = cx.string(hex_address);
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
