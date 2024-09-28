import { describe, test, expect } from "vitest";
import { decode, encode } from "../index.js";
describe("decode", () => {
	test("default", () => {
		expect(
			decode(
				"cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp",
			).hexAddress.toString("hex"),
		).toBe("85d80245dc02f5a89589e1f19c5c718e405b56cd");
		expect(decode("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp").netId).toBe(
			1029,
		);
		expect(decode("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp").type).toBe(
			"contract",
		);
		expect(
			decode(
				"CFX:ACC7UAWF5UBTNMEZVHU9DHC6SGHEA0403Y2DGPYFJP",
			).hexAddress.toString("hex"),
		).toBe("85d80245dc02f5a89589e1f19c5c718e405b56cd");

		expect(
			decode(
				"cfx:type.contract:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp",
			).hexAddress.toString("hex"),
		).toBe("85d80245dc02f5a89589e1f19c5c718e405b56cd");

		expect(
			decode(
				"cfx:type.contract:opt.random:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp",
			).hexAddress.toString("hex"),
		).toBe("85d80245dc02f5a89589e1f19c5c718e405b56cd");

		expect(
			decode(
				"cfx:aarc9abycue0hhzgyrr53m6cxedgccrmmyybjgh4xg",
			).hexAddress.toString("hex"),
		).toBe("1a2f80341409639ea6a35bbcab8299066109aa55"); // aa55

		expect(
			decode(
				"cfxtest:aan6jzja3gkvas5tm1j9uc623fj85j45hu6ec5g92p",
			).hexAddress.toString("hex"),
		).toBe("17c45500c993103b6f55d1f80b98c951eda35b3c"); // 5b3c

		expect(
			decode(
				"CFXTEST:TYPE.USER:AAMSC9WYG1E3UK0NFY5512HB1X8860HJGPMHUXV9YB",
			).hexAddress.toString("hex"),
		).toBe("14e17e5435c99826cb2d37bbe0e1bcfdee58e833"); // e833


    expect(
			decode("net10086:aaag4wt2mbmbb44sp6szd783ry0jtad5benr1ap5gp").hexAddress.toString("hex"),
		).toBe("006d49f8505410eb4e671d51f7d96d2c87807b09");// 0x0x006d…7b09

		expect(
			decode("net7876:aamue88kha4th1am7t3uvt94kr18t02xhac258u0wc").hexAddress.toString("hex"),
		).toBe("15027bc93834f3dc0aebf308bffa4b6fe7db1338");
	});

	test("invalid", () => {
		expect(() => decode("cfx:")).toThrow("Error: decode error invalid length");

		expect(() => decode("acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp")).toThrow(
			"Error: decode error zero or multiple prefixes",
		);

		expect(() =>
			decode("abc:aamue88kha4th1am7t3uvt94kr18t02xhac258u0wc"),
		).toThrow("Error: decode error invalid prefix");

		expect(() =>
			decode("cfx:aamue88kha4th1am7t3uvt94kr18t02xhac258u0wc"),
		).toThrow("Error: decode error invalid checksum");

		expect(() =>
			expect(decode("net7876:Aamue88kha4th1am7t3uvt94kr18t02xhac258u0wc")),
		).toThrow("Error: decode error mixed case string");
	});
});

describe("encode", () => {
	test("default", () => {
		expect(
			encode("85d80245dc02f5a89589e1f19c5c718e405b56cd", 1029, false),
		).toBe("cfx:acc7uawf5ubtnmezvhu9dhc6sghea0403y2dgpyfjp");
		expect(encode("85d80245dc02f5a89589e1f19c5c718e405b56cd", 1029, true)).toBe(
			"CFX:TYPE.CONTRACT:ACC7UAWF5UBTNMEZVHU9DHC6SGHEA0403Y2DGPYFJP",
		);

		expect(encode("0x85d80245dc02f5a89589e1f19c5c718e405b56cd", 1, false)).toBe(
			"cfxtest:acc7uawf5ubtnmezvhu9dhc6sghea0403ywjz6wtpg",
		);
		expect(encode("85d80245dc02f5a89589e1f19c5c718e405b56cd", 1, true)).toBe(
			"CFXTEST:TYPE.CONTRACT:ACC7UAWF5UBTNMEZVHU9DHC6SGHEA0403YWJZ6WTPG",
		);

		expect(
			encode("85d80245dc02f5a89589e1f19c5c718e405b56cd", 1111, false),
		).toBe("net1111:acc7uawf5ubtnmezvhu9dhc6sghea0403y11xjpva5");
		expect(
			encode("0x85d80245dc02f5a89589e1f19c5c718e405b56cd", 1111, true),
		).toBe("NET1111:TYPE.CONTRACT:ACC7UAWF5UBTNMEZVHU9DHC6SGHEA0403Y11XJPVA5");
	});

	test("invalid", () => {
		expect(() =>
			encode("85d80245dc02f5a89589e1f19c5c718e405b", 1029, false),
		).toThrow("Error: encode error: invalid length");
	});
});
