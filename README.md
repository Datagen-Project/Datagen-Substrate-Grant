<h1 align="center">
  <a href="https://www.b-datagray.com/"> 
    <img src="https://www.b-datagray.com/static/media/illustration-elements_token-logo.99d6bc5d.svg" height="200" width="200">
  </a>
  <br>
  Datagen Project
   <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/64146594/202314071-cd33a04e-17b7-4256-989e-a0c872ece713.png">
      <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/64146594/202314049-da656451-794f-4cb4-a6cc-291a7aa73103.png">
      <img alt="Show an image with black background" src="https://user-images.githubusercontent.com/64146594/202315277-9d969707-943e-417f-830a-4d1aca58ab47.jpg">
   </picture>
</h1>

## Index

- [Overview](#overview)
- [Milestones](#milestones)
- [Testing](#testing)


## Overview
We will implement only a PoC with this grant.

The goal is to achieve a fully functional mechanism for the random selection of the nodes in the fast blockchain and smooth communication between the two blockchains.

You can find more info on the project and about the grant visit our proposal at [this link](https://github.com/w3f/Grants-Program/blob/master/applications/Datagen_Project.md).

## Milestones 

- [Milestones - 1](#milestone-1---implement-the-randomized-substrate-pallet-pallet_random_node_selector-and-pallet_check_node_computational_work)
- [Milestones - 2](#milestone-2--connecting-the-two-blockchains)
- [Milestones - 3](#milestone-3--web-dapp)

### Milestone 1 - Implement the randomized substrate pallet `pallet_random_node_selector` and `pallet_check_node_computational_work`

STATUS: [DELIVERED](https://github.com/w3f/Grant-Milestone-Delivery/pull/608#issuecomment-1317485561)

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and an API specifications |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** on Medium that explains how we are going to develop the pallet. |
| 1. | Substrate pallet | We will create a `pallet_random_node_selector` that implement the randomized selection of the nodes for the fast blockchain using the Substrate `Randomness` trait. This pallet run on the Heavy Blockchain. |
|1a.| Functions | <ul><li>`reliable_node` update the list of the reliable nodes on the Heavy Blockchain.</li><li> `random_checker_node_selector` select 3 reliable random nodes in the fast blockchain to check the computational work.</li><li>`random_node_to_check` select a single random node to be check by the 3 checker nodes.</li></ul> 
|2.| Substrate pallet | We will create a `pallet_computational_work` that runs computational work on the fast nodes and pair them with their works.
|2a.| Functions | <ul><li>`math_work_testing` this function will provide math problems to solve by Fast Blockchain nodes, just for testing.</li><li>`hash_work` function will hash the raw math problem and the elaborated result from the node and pair, communicate to the Heavy Blockchain.</li></ul>
|3.| Substrate pallet| We will crate a `pallet_check_node_computational_work` that manage the control process on the Fast Blockchain.
|3a.| Functions | <ul><li>`check_computational_work` take info from the Heavy Blockchain (from the `pallet_random_node_selector`) and check the computational work of the target node. At this moment the nodes will make a simple math calculations just to check the mechanism.</li><li>`check_result` elaborate the result of the check process. If checked node has the same result of the majority of the checker nodes nothing happen. If the majority of the nodes have a different result from checked node this one will lose all his staked tokens (at this moment we only simulate the token lost) and checked node will be excluded from the Fast Blockchain.<li>`reliable_node` update the list of the reliable nodes on the Fast Blockchain.</li></ul>

### Milestone 2 — Connecting the two blockchains

STATUS: IN PROGREESS

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and and an documentation of the infrastructure |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** on Medium that explains how we are going to develop this step. |
| 1. | RPC Method (Random Selector) | We will create a custom RPC method to get the result of the random selection of the nodes to the Fast Blockchain. We will implement communication to get: <ul><li> Random node id to check and raw math problem (From HB to FB) </li><li>3 Random node id for the checkers and raw math problem (From HB to FB)</li></ul>|
| 2. | RPC Method (Blockchain status) | We will implement a set of RPC methods to check the status of the two blockchains. <ul><li>Mapping of all nodes an their status (reliable or not reliable) sync from Heavy Blockchain.</li><li>Computational works done and to be done by FB (total and mapping for every fast node)</li></ul> |
| 3. | Setup the two blockchains | We will setup the two blockchains to deep test the communications and `pallet_random_node_selector`, `pallet_check_node_computational_work` and `pallet_computational_work`.|


### Milestone 3 — Web Dapp

STATUS: TODO

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and and an documentation of the infrastructure |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** on Medium that explains how we are going to develop this step. |
| 1. | Web Dapp | We will create a web dapp to verify the functionality of the infrastructure, the GUI will display interactions between the two blockchains. |
| 1a.| Dapp Mock-up| Download the mock-up of the dapp at [this link](https://drive.google.com/drive/folders/1SJRPbczZhRaXVLHnLvmp_XIeBtBBv0g-). |
| 1b. | Home page | ![home page](https://i.imgur.com/MhQVfEj.png) <ol><li>Filter to switch between the two blockchains for searching purpose </li><li>Searching field (could search for blocks or nodes by typing id)</li><li>The id of the last node checked with `check_computational_work` pallet</li><li>The total number of nodes checked with `check_computational_work` pallet</li><li>Total checks with `check_computational_work`</li><li>Average number of checks on a single fast node with `check_computational_work` pallet</li><li>Id of a fast blockchain node</li><li>Number of checks on a node with `check_computational_work` pallet</li><li>The fast nodes that verify the computational work with `check_computational_work` pallet</li><li>Check result from `check_computational_work` pallet</li><li>Total blocks finalized by the blockchain</li><li>Total nodes of the blockchain</li><li>Block height</li><li>Block age</li><li>Validator id of the block</li></ol>|
| 1c. | Fast Blockchain - Block Page| ![FB Block](https://i.imgur.com/ZDMuX7I.png) <ol><li>Blockchain identifier</li><li>Block  identifier (height)</li><li>Block height, arrows change the block by 1 (left -1, right +1)</li><li>The age of the block and its creation time</li><li>Validator identifier, optionally a name and time required to validate the block</li><li>Total  number of  fast nodes at this block height</li><li>Number of nodes checked with `check_computational_work` pallet in this block</li><ol>|
| 1d. | Heavy Blockchain - Block Page | ![HB Block](https://i.imgur.com/iJCc6nh.png) For functionalities see 1c. list. |
| 1e. | Fast Blockchain - Node Page | ![FB Node](https://i.imgur.com/wOBlIdp.png) <ol><li>Node identifier</li><li>Node identifier arrows change the node by 1 (left -1, right +1)</li><li>Blockchain identifier</li><li>Last time node  checked with `check_computational_work` pallet.</li><li>Total number of checks with `check_computational_work` pallet on this node</li><li>How many pass results on this block</li><ol>|

## Testing

### Instruction for Milestone 1

You could build an image for the fast blockchain running this command in the fast_blockchain directory:

```shell
docker build -t fb-image .
```
Create the container with the fb-image and follow the instructions below:

The code is divided into two folders `fast_blockchain` and `heavy blockchain`.
Right now we don't have interaction between the two blockchain (this will happen in the M2) so you have to testing them separately.

#### Testing heavy_blockchain 

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

#### Testing fast_blockchain

You could build an image for the fast blockchain running this command in the fast_blockchain directory:

```shell
docker build -t fb-image
```
Create the container with the hb-image and follow the instructions below:

Navigate to `fast_blockchain/pallets/` adn run the below command:
```shell
cargo test
```

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

You can find a quick demo video at this [link](https://www.youtube.com/watch?v=4gwC2lOTazY)

Here a link to a medium article https://medium.com/@viacc/datagen-project-dev-blog-web3-fundation-milestone-1-b3ec2bdb1a95
where you can check the description of the First Milestone

## Licensing

The code in this project is licensed under [GNU general Public License v3.0](https://github.com/Datagen-Project/DataGen-Smart-Contracts/blob/main/LICENSE.md).
