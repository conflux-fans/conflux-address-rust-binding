import { bench } from "vitest";
import { decode, encode } from "../index";

bench("decode", () => {
  decode("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp");
});

bench("encode", () => {
  encode("85d80245dc02f5a89589e1f19c5c718e405b56cd", 1, false);
});
