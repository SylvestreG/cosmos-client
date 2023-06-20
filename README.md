# CosmosClient

CosmosClient is a Rust RPC and signing client for Cosmos SDK, inspired by cosmjs in JavaScript. It is built on top of
cosmrs and aims to provide a simplified and user-friendly interface for interacting with the Cosmos SDK blockchain.

## Features

- Simplified API: CosmosClient strives to provide an intuitive and easy-to-use API, making it straightforward to
  interact with the Cosmos SDK.
- RPC Functionality: Interact with the blockchain by making RPC calls to query blockchain data, such as account
  balances, transaction information, and more.
- Transaction Signing: Sign transactions locally using private keys, providing a secure way to send transactions to the
  blockchain.
- Seamless Integration: CosmosClient is designed to integrate seamlessly with the Cosmos SDK ecosystem, allowing
  developers to build Rust applications with ease.

## Installation

To use CosmosClient in your Rust project, add the following line to your Cargo.toml file:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
cosmos_client = "0.1"
```

Getting Started
Here's a quick example to get you started with CosmosClient:

```rust
use cosmos_client::client::Rpc;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::Coin;
use cosmos_client::error::CosmosClient;
use cosmos_client::signer::Signer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), CosmosClient> {
    // ask for 24 words
    env_logger::init();

    let mut client = Rpc::new("https://rpc-cosmoshub-ia.cosmosia.notional.ventures/").await?;
    let signer = Signer::from_mnemonic("PUT your 24 words Here", "cosmos", "uatom", None, 30, 25_000)?;
    let address = signer.public_address.to_string();
    client.attach_signer(signer).await?;

    println!("signer loaded for {address}");

    let response = client
        .send(
            address.as_str(),
            vec![Coin {
                denom: "uatom".to_string().parse()?,
                amount: 1_000_000.to_string(),
            }],
            None,
        )
        .await?;
    println!("response {response:#?}");

    Ok(())
}
```

Please note that this is a simplified example and may not cover all available functionality.
Refer to the documentation and examples for more advanced usage and features.

# Documentation

For detailed usage instructions, API reference, and examples, please refer to the CosmosClient Documentation.

## Contributing

Contributions to CosmosClient are welcome! If you would like to contribute, please follow these steps:

- Fork the repository and clone it to your local machine.
- Create a new branch for your feature or bug fix.
- Implement your changes and ensure that the code passes all tests.
- Write clear and concise commit messages.
- Push your branch to your forked repository.
- Open a pull request with a description of your changes.

## License

CosmosClient is distributed under the MIT License. See the LICENSE file for more information.

## Acknowledgments

CosmosClient is built on top of the cosmrs library, and we would like to express our gratitude to the authors and
contributors of cosmrs for their work and contributions to the Rust Cosmos SDK ecosystem.

## Contact

If you have any questions, suggestions, or feedback, please feel free to reach out to us at

Happy coding with CosmosClient!