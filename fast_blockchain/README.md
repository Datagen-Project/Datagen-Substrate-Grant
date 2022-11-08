# Testing fast_blockchain

You could build an image for the fast blockchain running this command in the fast_blockchain directory:

```shell
docker build -t fb-image .
```
Create the container with the fb-image and follow the instructions below:

Navigate to `fast_blockchain/pallets/` adn run the below command:
```shell
cargo test
```
This command will run the tests for the `computational_work` and `check_node_computational_work` pallets.

You could also run the blockchain and make some manual tests with the [Polkadot.js](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944) web app, this time you have to run more nodes to simulate the blockchain.

Run at least 4 nodes run every following commands in a new session:

Node 1 - Alice
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator1 \
--alice \
--node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a \
--port 30333 \
--ws-port 9944
```
Node 2 - Bob
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator2 \
--bob \
--node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 \
--port 30334 \
--ws-port 9945
```
Node 3 - Charlie
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator3 \
--charlie  \
--node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
--port 30335 \
--ws-port=9946 \
--offchain-worker always
```
Node 4 - Dave
```shell
./target/release/node-template \
--chain=local \
--base-path /tmp/validator4 \
--dave \
--node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a \
--port 30336 \
--ws-port 9947 \
--offchain-worker always
```

Now your blockchain should produce 1 block every second.

Select `computationalWork` pallet from the extrinsics.

To submit computational work (an easy math work in this case for testing) call `hash_work`.

To simulate malicious intent every computational work or check on a block that is a multiply of 5 should be a wrong number (0).
For example if you call `hash_work` on the 175th block it should submit 0 as computational work and it should be check as invalid and malicious submission.

Same thing for the checkers that could check with malicious intent.

Go to the network event you should see a `computationalWork.ResultsComputationalWork` with some info about the computational work, see more details in the inline documentation in the test code.
Then you should see 3 `checkNodeComputationalWork.CheckResult` event triggered by the computational work submission, and a `checkNodeComputationalWork.FinalResult` event with the final result about the checking process.

If the 2/3 of the checkers agree with the checked node you should see a true as `is_passed` value in the `FinalResult` event.

You could also set every how many computational work the network should check the work.
To do that call the `computationalWork` extrinsics `setCheckEveryXWorks(x)` and set the index.

remember to delate the temp files for every validator if you want to rerun the tests with:
```shell
$ ./target/release/node-template purge-chain --base-path /tmp/validator1 --chain local
$ ./target/release/node-template purge-chain --base-path /tmp/validator2 --chain local
$ ./target/release/node-template purge-chain --base-path /tmp/validator3 --chain local
$ ./target/release/node-template purge-chain --base-path /tmp/validator4 --chain local
```

You could find a quick demo video at this [link](https://www.youtube.com/watch?v=4gwC2lOTazY)