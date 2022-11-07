# Testing heavy_blockchain

You could build an image for the heavy blockchain running this command in the heavy_blockchain directory:

```shell
docker build -t hb-image
```
Create the container with the hb-image and follow the instructions below:

Navigate to `heavy_blockchain/pallets/` and run the below command:
```shell
cargo test
```
This command will run the tests for the `random_node_selector` pallet.

You could also run the blockchain and make some manual tests with the [Polkadot.js](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944) web app, run the commands:
```shell
$ cargo build --release
$ ./target/release/node-template --dev
```
Then connect to the he [Polkadot.js](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944) web app.

To select a random node submit the extrinsic of the `randomNodeSelector` called `randomNodeToCheck`.
To select the 3 random node that will be the checkers call `randomCheckerNodeSelector`.

Now you could visualize the event in the network inspector, note at the moment we have also an empty peerId value, this is correct we add this information for possible future implementation but we will use the owner account to do the check at the grant.
