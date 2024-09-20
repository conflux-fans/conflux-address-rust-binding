# conflux-address-rust

This is a node.js module binding to [cfx-addr](https://crates.io/crates/cfx-addr).

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing conflux-address-rust

Installing conflux-address-rust requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install @conflux-dev/conflux-address-rust
```

This fully installs the project, including installing any dependencies and running the build.

## How to use

After installing, you can explore its exports as below:

```js
const {encode, decode} = require('@conflux-dev/conflux-address-rust');

decode('cfxtest:aak2rra2njvd77ezwjvx04kkds9fzagfe6d5r8e957');
/* {
  hexAddress: '0x1386b4185a223ef49592233b69291bbe5a80c527',
  netId: 1,
  type: 'user'
} */

encode('0x1386b4185a223ef49592233b69291bbe5a80c527', 1, true)
// 'CFXTEST:TYPE.USER:AAK2RRA2NJVD77EZWJVX04KKDS9FZAGFE6D5R8E957'
encode('0x1386b4185a223ef49592233b69291bbe5a80c527', 1, false)
// 'cfxtest:aak2rra2njvd77ezwjvx04kkds9fzagfe6d5r8e957'
encode('0x1386b4185a223ef49592233b69291bbe5a80c527', 1, true)
// 'CFXTEST:TYPE.USER:AAK2RRA2NJVD77EZWJVX04KKDS9FZAGFE6D5R8E957'
encode('0x1386b4185a223ef49592233b69291bbe5a80c527', 1029, true)
// 'CFX:TYPE.USER:AAK2RRA2NJVD77EZWJVX04KKDS9FZAGFE6KU8SCZ91'
```

## API

### encode

#### params

* `address(string)`: hex40 address
* `netId(number)`: network id
* [`verbose(bool)`]: whether encode as verbose

#### return

* `address(string)`: base32 address

### decode

#### params

* `address(string)`: base32 encoded address

#### return

* `hexAddress(string)`: buffer
* `netId(number)`: network id
* `type(string)`: address type

## Contribute

In the project directory, you can run:

### `yarn install`

Installs the project, including running `yarn build`.

### `yarn build`

After yarn build/npm run build command, you can see package-template.[darwin|win32|linux].node file in project root. This is the native addon built from [lib.rs](/src/lib.rs).


### Test in local

- yarn
- yarn build
- yarn test

And you will see:

```bash
 ✓ __test__/index.spec.ts (4)
   ✓ decode (2)
     ✓ default
     ✓ invalid
   ✓ encode (2)
     ✓ default
     ✓ invalid
```