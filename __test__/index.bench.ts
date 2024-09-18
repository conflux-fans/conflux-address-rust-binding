import { bench } from "vitest";
import { decode } from "../index";

bench("decode", () => {
  decode("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp");
});
