import { describe, test, expect } from "vitest";
import { decode } from "../index.js";
describe("decode", () => {
  test("default", () => {
    expect(decode("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp"))
      .toMatchInlineSnapshot(`
        {
          "hexAddress": "0x0x85d8…56cd",
          "netId": 1029,
          "type": "contract",
        }
      `);

    expect(
      decode("CFX:ACC7UAWF5UBTNMEZVHU9DHC6SGHEA0403Y2DGPYFJP")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x85d8…56cd",
        "netId": 1029,
        "type": "contract",
      }
    `);

    expect(
      decode("cfx:type.contract:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x85d8…56cd",
        "netId": 1029,
        "type": "contract",
      }
    `);

    expect(
      decode(
        "cfx:type.contract:opt.random:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp"
      )
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x85d8…56cd",
        "netId": 1029,
        "type": "contract",
      }
    `);

    expect(
      decode("cfx:aarc9abycue0hhzgyrr53m6cxedgccrmmyybjgh4xg")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x1a2f…aa55",
        "netId": 1029,
        "type": "user",
      }
    `);
    expect(
      decode("cfxtest:aan6jzja3gkvas5tm1j9uc623fj85j45hu6ec5g92p")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x17c4…5b3c",
        "netId": 1,
        "type": "user",
      }
    `);

    expect(
      decode("CFXTEST:TYPE.USER:AAMSC9WYG1E3UK0NFY5512HB1X8860HJGPMHUXV9YB")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x14e1…e833",
        "netId": 1,
        "type": "user",
      }
    `);

    expect(
      decode("net10086:aaag4wt2mbmbb44sp6szd783ry0jtad5benr1ap5gp")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x006d…7b09",
        "netId": 10086,
        "type": "builtin",
      }
    `);
    expect(
      decode("net7876:aamue88kha4th1am7t3uvt94kr18t02xhac258u0wc")
    ).toMatchInlineSnapshot(`
      {
        "hexAddress": "0x0x1502…1338",
        "netId": 7876,
        "type": "user",
      }
    `);
  });
});
