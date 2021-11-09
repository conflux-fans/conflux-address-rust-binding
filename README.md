# conflux-address-rust

This is a node.js module binding to [cfx-addr](https://crates.io/crates/cfx-addr).

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing conflux-address-rust

Installing conflux-address-rust requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install conflux-adress-rust
```

This fully installs the project, including installing any dependencies and running the build.

## How to use

After installing, you can explore its exports at the Node REPL:

```js
const {encode, decode} = require('conflux-address-rust');

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
* `verbose(bool)`: whether encode as verbose

#### return

* `address(string)`: base32 address

### decode

#### params

* `address(string)`: base32 encoded address

#### return

* `hexAddress(string)`: hex40 address
* `netId(number)`: network id
* `type(string)`: address type

## Contribute

In the project directory, you can run:

### `npm install`

Installs the project, including running `npm run build`.

### `npm build`

Builds the Node addon (`index.node`) from source.

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm build` and `npm build-*` commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm build-debug`

Alias for `npm build`.

#### `npm build-release`

Same as [`npm build`](#npm-build) but, builds the module with the [`release`](https://doc.rust-lang.org/cargo/reference/profiles.html#release) profile. Release builds will compile slower, but run faster.

### `npm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).


