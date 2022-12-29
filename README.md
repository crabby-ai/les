## **Les.rs** - Rust Cryptocurrency Exchange Library

An open source Rust high performance cryptocurrency trading API with support for multiple exchanges and language wrappers. written in rust(🦀) with ❤️.
This library provides a simple interface for interacting with cryptocurrency exchanges from Rust. It supports a wide range of exchanges and makes it easy to access their APIs and perform common tasks such as getting account balances and placing orders.

## **Installation**

To install the library, add the following to your **Cargo.toml** file:

Copy code

`[dependencies] les = "0.1"`

You can then use the library in your Rust code by adding the following to your **main.rs** or **lib.rs** file:

Copy code

`extern crate les;`

## **Supported Exchanges**

The following exchanges are currently supported by the library:

| Exchange | Available |
| --- | --- |
| Binance | ✅ |
| Coinbase | ⬜️ |
| Bitfinex | ⬜️ |
| Kraken | ⬜️ |

More exchanges will be added in future updates.

## **Usage**

To use the library, you will need to create an instance of the **Exchange** struct, passing in the name of the exchange you want to use as a string. You will also need to provide your API key and secret key, which you can obtain from the exchange.

Copy code

```rust
use les::Exchange; let exchange = Exchange::new("binance", "api_key", "secret_key");
```

You can then use the **exchange** object to call various methods to access the exchange's API. For example, to get the current balance of your account:

Copy code

```rust
let balance = exchange.get_balance().unwrap();
```

See the documentation for a full list of available methods and their usage.

## **Documentation**

Full documentation for the library can be found [here](https://docs.rs/les).

## **Contributing**

We welcome contributions to the library! If you would like to contribute, please follow these steps:

1.  Fork the repository.
2.  Create a new branch for your changes.
3.  Make your changes and commit them to your branch.
4.  Push your branch to your fork on GitHub.
5.  Submit a pull request to the **main** branch of the **crabby-ai/les** repository.

Please make sure to follow the coding style and conventions used in the rest of the codebase, and make sure to test your changes thoroughly before submitting the pull request.

## **License**

This library is released under the MIT License. See the [LICENSE](https://chat.openai.com/chat/LICENSE) file for details.

## **Acknowledgements**

I would like to thank the following individuals and organizations for their contributions to this project:

*   The Rust community for their support and guidance.
*   The developers of the various cryptocurrency exchanges provide APIs that allow us to access their platforms.
