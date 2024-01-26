# 依赖分析

> 整个项目的依赖已经使用`cargo tree`生成，位于根目录下的`dependency-tree.txt`中。

查看根目录的`Cargo.toml`中的内容可知，该项目由三个成员组成，分别是node，runtime和pallet。

根目录中的`Cargo.lock`记录了全局依赖的crates。该文件中包含的内容过多，因此笔者计划从三个子成员的`Cargo.toml`入手，研究其依赖。

> 观前提示：根据笔者的经验，以下crate的命名有一些规律，能帮助我们更快找到源代码的位置。
> - 若以`frame`开头，那么多半在远程仓库的`frame`目录下。
> - 若以`sp`开头，那么多半在远程仓库的`primitives`目录下。
> - 若以`sc`开头哦，那么多半在远程仓库的`client`目录下。

## pallet子成员的依赖

由于Substrate Node Template的pallet仅有一个模板pallet，因此其依赖关系最为简单。打开`pallet/Cargo.toml`可见，其仅依赖于以下crates：

- [parity-scale-codec](https://crates.io/crates/parity-scale-codec)：使用[SCALE](https://docs.substrate.io/reference/scale-codec/)编解码方案，可以把它视为轻量版且不依赖于std的serde。不过它并不会带有被编码数据的类型信息，这意味着在编解码时，环境需要知道被编码数据的具体类型信息。好在我们的编解码都在Rust环境中完成，所以可以放心使用它。
- [scale-info](https://crates.io/crates/scale-info)：似乎是配合SCALE编解码方案使用的，它能够提供类型信息，配合SCALE编解码器实现更精确的编解码。
- [frame-benchmarking](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/benchmarking)：评估一个交易（extrinsic）的执行耗时，以此决定是否要执行这个交易还是丢弃它。可以被用来缓解DoS攻击。
- [frame-support](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/support)：官方仓库并没有很详细的解释，只说对运行时runtime提供了支持。
- [frame-system](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/system)：为其他Pallet提供底层支持。根据仓库的说法，它定义了一些核心数据结构、一些函数和管理存储项（storage items）的机能。
- [sp-core](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/core)：一点介绍信息都没有。但[Doc.rs上对sp-core的描述](https://docs.rs/sp-core/latest/sp_core/)详细列举了它向外导出的模块、宏、结构体等信息。
- [sp-io](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/io)：[Doc.rs上对sp-io的描述](https://docs.rs/sp-io/latest/sp_io/)是“runtime对外沟通的接口”，这些接口被称为主机函数（host function）。
- [sp-runtime](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/runtime)：[Doc.rs上对sp-runtime的描述](https://docs.rs/sp-runtime/latest/sp_runtime/)表明，这个crate包含了一大堆数据结构，且不只是FRAME用到的数据结构。

## runtime子成员的依赖

该子成员新增了下列依赖：

- [frame-try-runtime](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/try-runtime)：在官方文档上非常神秘，但[Substrate Dev Hub Sandbox](https://lsgunnlsgunn.github.io/dev-hub-sandbox-tabs/reference/command-line-tools/try-runtime/)上居然有更多的信息。根据后者的描述，try-runtime这个玩意能够捕获runtime的快照，并在这个快照上编写测试来debug。
- [frame-executive](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/executive)：接收外来的交易请求并根据这些交易想进行的操作分派给不同的runtime中的模块执行。
- [frame-system-rpc-runtime-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/system/rpc/runtime-api)：提供了一个`frame_system_rpc_runtime_api::AccountNonceApi`的trait，具体的作用没看懂，只从名字上推断和RPC有关系。。。
- [frame-system-benchmarking](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/system/benchmarking)：System pallet底下的子pallet。作用未知，构成内容未知。
- [pallet-aura](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/aura)：Aura是Substrate使用的共识算法（？）。这个crate扩展了它的功能，但具体扩展了啥没太看懂。据称，它提供了一个公开的`slot_duration`函数，推测是能够修改Aura算法中时间槽（time slot）的长度吧。
- [pallet-balances](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/balances)：管理账户和余额所使用的crate。
- [pallet-grandpa](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/grandpa)：为Grandpa共识算法提供支持。这种共识算法经常和BABE共识算法一起使用。关于Grandpa共识算法，请见[Polkadot共识第2部分：Grandpa](https://polkadot.network/blog/polkadotgong-shi-di-2bu-fen-grandpa)
- [pallet-sudo](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/sudo)：指定**一个**账号作为sudo key账号，用该账号可以执行需要管理员权限的函数。sudo key账号也可以指定其他账号做新的sudo key，自己光荣退休。
- [pallet-transaction-payment-rpc-runtime-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/transaction-payment/rpc/runtime-api)：transaction payment pallet的runtime API定义
- [pallet-timestamp](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/timestamp)：提供为区块设置时间戳和获取当前区块时间戳的机能。
- [pallet-transaction-payment](https://lib.rs/crates/pallet-transaction-payment)：看上去似乎是用来计算交易时的一些手续费的。这包含三部分费用：交易本身的weight fee、按交易编码结果的长度支付的length fee和小费tip（使用小费可以更优先进入交易队列）。
- [sp-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/api)：在[Doc.rs](https://docs.rs/sp-api/latest/sp_api/)上的解释是<kbd>作为node和runtime之间的桥梁</kbd>。
- [sp-block-builder](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/block-builder)：提供了一个叫做`BlockBuilder`的`trait`，规定了构造区块所需要提供的一些函数。
- [sp-consensus-aura](https://docs.rs/sp-consensus-aura/latest/sp_consensus_aura/)：啥也没看出来，只知道和Aura共识算法应该是有关的。不过[Doc.rs](https://docs.rs/sp-consensus-aura/latest/sp_consensus_aura/)上倒是记录了一些信息，可以参考一下。
- [sp-consensus-grandpa](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/consensus/grandpa)：同上。[Doc.rs](https://docs.rs/sp-consensus-grandpa/latest/sp_consensus_grandpa/)
- [sp-inherents](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/inherents)：Substrate中存在一些inherent（固有组件），它们是Substrate的核心，因此不能被裁剪掉。这个crate的功能就是提供一些类型和trait，方便创建和检查inherent。例如在runtime中，如果一个模块想创建inherent，就得实现`ProvideInherent`的trait。
- [sp-offchain](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/offchain)：Offchain Worker，只提供了一个叫`OffChainWorkerApi`的trait。根据Google Bard的说法，Offchain Worker就是指在链外部执行的操作，例如一个程序从链上收取信息，然后在链外部进行机器学习，最后可能还会运行学到的模型生成一些新的数据，再推送回链上。Offchain Worker通过RPC接口与链通信。
- [sp-session](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/session)：Session（会话）是Substrate中的重要概念。根据Google Bard的说法，每个网络参与者都持有一个会话ID，通过认证之后即可参与到会话中。若在会话中做出不诚实行为，则将遭受惩罚，严重时将会被没收所有质押的财产。
- [sp-std](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/std)：可以在std和no-std环境中向外导出一些std中的数据结构供外部使用，例如`Vec`啥的。
- [sp-transaction-pool](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/transaction-pool)：交易池相关的元语和runtime API。在其子模块中提供了`sp_transaction_pool::runtime_api::TaggedTransactionQueue`的trait，要求实现验证交易合法性的方法。
- [sp-version](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/version)：提供一个函数，调用后返回runtime的版本信息。
- [substrate-wasm-builder](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/utils/wasm-builder)：将项目编译为WASM可执行文件时使用。

## node子成员的依赖

相比起上两者，node子成员的依赖有如下增添：

- [clap](https://crates.io/crates/clap)：命令行参数解析工具。
- [futures](https://crates.io/crates/futures)：Rust官方维护的异步库。
- [jsonrpsee](https://crates.io/crates/jsonrpsee)：异步的JSON-RPC库。
- [try-runtime-cli](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/utils/frame/try-runtime/cli)：一点介绍信息都没有。
- [substrate-build-script-utils](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/utils/build-script-utils)：为`build.rs`服务。
- [substrate-frame-rpc-system](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/utils/frame/rpc/system)：和FRAME关系密切的RPC操作。
- [frame-benchmarking-cli](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/utils/frame/benchmarking-cli)：允许在命令行中执行基准测试。
- [pallet-transaction-payment-rpc](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/frame/transaction-payment/rpc)：Transaction Payment Pallet的RPC接口。
- [sc-cli](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/cli)：只说是Substrate的CLI库。
- [sc-executor](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/executor)：提供向runtime发送执行命令请求的机能的库，但具体作用看不懂。
- [sc-network](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/network)：Substrate的P2P网络库。
- [sc-service](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/service)：超模！据仓库readme所述，它启动一个线程，在这个线程中启动network模块、client模块和交易池（extrinsic pool）模块，并管理它们之间的通信。
- [sc-telemetry](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/telemetry)：没太看懂，但似乎是能把一些遥测数据发送到遥测服务器，最后应该是会在[Polkadot Telemetry](https://telemetry.polkadot.io/)中显示出来。
- [sc-transaction-pool](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/transaction-pool)：提供client模块中的交易池支持。难得在仓库页面看到这么详细的介绍。
- [sc-transaction-pool-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/transaction-pool/api)：从名字上看应该是为上面的crate提供API支持的。
- [sc-offchain](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/offchain)：看着和`sp-offchain`很像，都是为Offchain Worker服务的。
- [sc-statement-store](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/statement-store)：看名字似乎是存储链上状态的。
- [sc-consensus-aura](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/consensus/aura)：看上去和`sp-consensus-aura`很像，都是为Aura共识算法服务的。
- [sc-consensus-grandpa](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/consensus/grandpa)：看上去和`sp-consensus-grandpa`很像，都是为Grandpa共识算法服务的。
- [sc-consensus](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/consensus)：看上去是Substrate共识算法的大合集，里面有包括但不限于PoW，Aura，Grandpa，babe等共识算法的子crate。
- [sc-client-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/api)：client接口。。。？至少仓库页面是这么写的，具体作用未知。
- [basic-authorship](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/basic-authorship)：根据其readme中的"Basic implementation of block-authoring logic"描述，似乎和区块创建有关？
- [sc-rpc-api](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/client/rpc-api)：
- [sp-keyring](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/keyring)：根据仓库readme介绍，它内置了多个测试用的账号。
- [sp-blockchain](https://github.com/paritytech/substrate/tree/polkadot-v1.0.0/primitives/blockchain)：区块链相关的trait。

# 项目分析

经过一通排查，终于可以调试整个项目啦！具体的配置请见`.vscode/launch.json`中。

## node子成员

可以肯定的是，`node/src/main.rs`中的`main()`就是整个Substrate Node Template项目的入口函数。证据如下：在`main()`中加入调试语句，编译运行后该调试语句总是比其他输出先打印出来。将`main()`函数中的`command::run()`注释掉之后，再编译运行，那么节点将只会输出调试语句的内容。

另外一点需要注意的是，该`main()`的返回值`sc_cli::Result<T>`实际上是如下的包装：
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

### command::run()

位于`node/src/command.rs`中。环境变量的种类很多，但全部由`node/src/cli.rs`中的`Subcommand`枚举类型定义。每种环境变量具体的内容又各自有定义，将鼠标移动到标识符上方查看悬浮提示。

`command::run()`中利用一个`match`匹配这些不同类型的环境变量。若什么环境变量都没发现，那么程序将进入`None`分支。根据调试的结果来看，我们的程序总是会进入None分支。在这个分支下，从命令行传入的参数将得到处理。

# 有用的资料

## 查找crate信息

- [crates.io](https://crates.io/)
- [lib.rs](https://lib.rs/)

> 在查找时善用`no-std`标签，对于查找没有std依赖的crate非常有用。

## 代码阅读帮助

- [Explore the code | Substrate](https://docs.substrate.io/quick-start/explore-the-code/)
- [Substrate Dev Hub Sandbox](https://lsgunnlsgunn.github.io/dev-hub-sandbox-tabs/)

## Polkadot共识

- [Part 1 简介](https://polkadot.network/blog/polkadotgong-shi-di-1bu-fen-jian-jie)
- [Part 2 Grandpa共识算法](https://polkadot.network/blog/polkadotgong-shi-di-2bu-fen-grandpa)
- [Part 3 BABE](https://polkadot.network/blog/polkadotgong-shi-di-3bu-fen-babe)
- [Part 4 安全性](https://polkadot.network/blog/polkadotgong-shi-di-4bu-fen-an-quan-xing)

# Substrate Node Template

A fresh [Substrate](https://substrate.io/) node, ready for hacking :rocket:

A standalone version of this template is available for each release of Polkadot in the [Substrate Developer Hub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/) repository.
The parachain template is generated directly at each Polkadot release branch from the [Node Template in Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) upstream

It is usually best to use the stand-alone version to start a new project.
All bugs, suggestions, and feature requests should be made upstream in the [Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) repository.

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this template.
Check the [Install](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.
Alternatively, you can use one of the [alternative installation](#alternatives-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/node-template -h
```

You can generate and view the [Rust Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/node-template --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/node-template purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that includes several prefunded development accounts.


To persist chain state between runs, specify a base path by running a command similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, see [Simulate a network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
  Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
  Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain.
  Take note of the `development_config` and `testnet_genesis` functions,.
  These functions are used to define the genesis state for the local development chain configuration.
  These functions identify some [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/) and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation.
  Take note of the libraries that this file imports and the names of the functions it invokes.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models) such as Aura for block authoring and GRANDPA for finality.



### Runtime

In Substrate, the terms "runtime" and "state transition function" are analogous.
Both terms refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define.
The Substrate project in this repository uses [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) to construct a blockchain runtime.
FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".
At the heart of FRAME is a helpful [macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to create pallets and flexibly compose them to create blockchains that can address [a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note the following:

- This file configures several pallets to include in the runtime.
  Each pallet configuration is defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the [`construct_runtime!`](https://paritytech.github.io/substrate/master/frame_support/macro.construct_runtime.html) macro, which is part of the [core FRAME pallet library](https://docs.substrate.io/reference/frame-pallets/#system-pallets).

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with [the Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is comprised of a number of blockchain primitives, including:

- Storage: FRAME defines a rich set of powerful [storage abstractions](https://docs.substrate.io/build/runtime-storage/) that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://docs.substrate.io/build/events-and-errors/) to notify users of significant state changes.
- Errors: When a dispatchable fails, it returns an error.

Each pallet has its own `Config` trait which serves as a configuration interface to generically define the types and parameters it depends on.

## Alternatives Installations

Instead of installing dependencies and building this source directly, consider the following alternatives.

### Nix

Install [nix](https://nixos.org/) and
[nix-direnv](https://github.com/nix-community/nix-direnv) for a fully plug-and-play
experience for setting up the development environment.
To get all the correct dependencies, activate direnv `direnv allow`.

### Docker

Please follow the [Substrate Docker instructions here](https://github.com/paritytech/substrate/blob/master/docker/README.md) to build the Docker container with the Substrate Node Template binary.
