# Substrate Node Template学习笔记

## 可能有用的资源链接

- [CLI Tools中对Substrate Node Template的描述](https://docs.substrate.io/reference/command-line-tools/node-template/)
- [Explore the code](https://docs.substrate.io/quick-start/explore-the-code/)

## 阅读`cargo metadata`的输出

该指令的输出格式为JSON格式，整体还是较为清晰的。只不过其中的`resolve`字段，比较不好理解，现结合Cargo手册进行说明。

`resolve`字段下设`nodes`和`root`两个字段。后者是根crate，但不一定有值，可能是`null`；前者是一个数组，标识依赖图（Dependency Graph）中的节点，其中包含许多形似如下的代码片段：

```json
{
    // 这id确实能唯一标识一个依赖项
    "id": "registry+https://github.com/rust-lang/crates.io-index#aead@0.4.3",
    // 当前节点所有依赖项的id
    "dependencies": [
        "registry+https://github.com/rust-lang/crates.io-index#generic-array@0.14.7",
        "registry+https://github.com/rust-lang/crates.io-index#rand_core@0.6.4"
    ],
    // 当前节点依赖项的具体信息，和上边的"dependencies"的内容一一对应
    "deps": [
        {
            // 考虑到Cargo.toml中可以给crate改别名，
            // 因此这儿的name有可能是别名
            "name": "generic_array",
            // 依赖项的id
            "pkg": "registry+https://github.com/rust-lang/crates.io-index#generic-array@0.14.7",
            "dep_kinds": [
                {
                    // 依赖项的类型，可以是
                    // "normal"（表现为null）、"build"、"dev"等
                    "kind": null,
                    // 依赖项的目标平台，可以是
                    // null（表现为null）、"cfg(windows)"等
                    "target": null
                }
            ]
        },
        {
            "name": "rand_core",
            "pkg": "registry+https://github.com/rust-lang/crates.io-index#rand_core@0.6.4",
            // 其余内容和上一项类似
            // -- snip --
        }
    ],
    // 当前节点已经启用的feature列表
    "features": [
        "alloc",
        "rand_core",
        "std"
    ]
}
```

## Substrate Node Template及其大致划分

[Explore the code](https://docs.substrate.io/quick-start/explore-the-code/)章节对Substrate Node Template（下简称SNT）的代码结构作了概要性陈述，翻译如下：

> **关于SNT**
>
> SNT囊括了一些默认的区块链要素构件，例如p2p网络、简易共识机制、交易处理等。针对账号、余额、交易手续费、管理员权限等功能，SNT也提供了一些基础的功能支持。
>
> 这些核心功能的实现，是通过预先定义好的**pallets**模块来实现的，例如以下几个模块：
> - `pallet_balances`：管理账户资产和账户之间的转账。
> - `pallet_transaction_payment`：管理交易手续费的处理。
> - `pallet_sudo`：执行需要管理员权限的操作。
>
> SNT也提供了一个模板pallet——`pallet_template`，用以展示如何实现自定义pallet的功能。
>
> 有了上述的了解后，我们可以更加深入地探索SNT的代码，看看`substrate-node-template`里都有些啥。
>
> **清单文件**
>
> Substrate基于Rust编写，因此每个Rust项目都会有自己独立的`Cargo.toml`文件，指导该项目的编译过程。在`substrate-node-template`目录下的`Cargo.toml`，记录着构成SNT 工作空间（workspace）的几个成员（member）项目，像这样：
>
> ```toml
> [workspace]
> members = [
>     "node",
>     "pallets/template",
>     "runtime",
> ]
> [profile.release]
> panic = "unwind"
> ```
>
> 这么看来，SNT是由三个成员组成的：
> - `node`：提供Rust模块实现了很多核心区块链服务，例如p2p网络、区块产生、区块确认（finalization）、交易池管理等。
> - `pallets/template`：提供了模板pallet——`pallet_template`，用以展示如何实现自定义pallet的功能。
> - `runtime`：提供区块链应用逻辑，包括账号、余额、交易手续费等功能的实现。
>
> 每个成员项目也有各自的`Cargo.toml`清单文件，内含编译各成员所需的依赖项、设定等信息。举例来说，`node`项目的`Cargo.toml`文件指定了该成员的名字叫"node-template"，并且列出了一些核心库和原语，以提供区块链节点模板提供基本区块链服务所需的基本功能。关于库和原语，在[架构与rust库](https://docs.substrate.io/learn/architecture/)中有更详细的描述。
>
> 当下，只需明白清单文件记录着很多重要信息，就足够了。
>
> 如果去看`runtime/Cargo.toml`和`pallets/template/Cargo.toml`的话，会发现他们依赖的库和原语不尽相同，但是对各自依赖些啥会有点了解。
>
> **核心客户端源码**
>
> Substrate区块链的一大特点就是，节点由两个部分组成：核心客户端和运行时。SNT也不例外，其提供核心客户端服务的rust项目位于`node/src`目录，而运行时实现位于`runtime/src`目录。
>
> 默认情况下，`node/src`目录包含以下Rust模块：
>
> - `benchmarking.rs`
> - `chain_spec.rs`
> - `cli.rs`
> - `command.rs`
> - `lib.rs`
> - `main.rs`
> - `rpc.rs`
> - `service.rs`
>
> 大部分核心客户端服务逻辑都位于`service.rs`模块中。这些代码在开发时几乎不需要更改。
>
> 在开发时最有可能需要修改的是`chain_spec.rs`文件，它描述了默认开发和本地测试网络的配置，包括默认预充值的开发用账户，和预置的有生产区块权限的节点。如果开发者想创建一个自定义的链，那么就需要修改`chain_spec.rs`文件，以指定该链所连接到的网络，以及与之通信的其他节点。
>
> **SNT默认运行时**
>
> 鉴于Substrate的模块性和灵活性，你可以任意修改工作空间中的任意一个Rust项。但是，大部分应用开发工作都在运行时和pallet中进行。在开始自定义运行时之前，你应该花点时间探索一下SNT默认的运行时。
>
> 默认运行时的清单文件`Cargo.toml`中记录了许多名字类似于`pallet-balances`, `pallet-sudo`这样的依赖项。除此之外，也有些名字类似`frame-xxx`的核心依赖项，例如`frame-system`, `frame-support`, `frame-executive`等。关于这些[核心依赖项](https://docs.substrate.io/learn/runtime-development/#core-frame-services)，目前只需知道它们是必要的就行了，不用太过在意。
>
> 默认运行时的源代码放在`runtime/src/lib.rs`文件中。打开一看好复杂，其实本质就这样：
>
> - import了`frame_system`和`frame_support`核心依赖项。
> - 指定了运行时的版本信息。
> - 声明了要包含的pallet。
> - 声明了每个pallet的类型和参数。
> - 设置了每个pallet的常量和变量值。
> - 为每个pallet实现了`Config` trait。
> - 用这些pallet构造出了运行时。
> - 为评估pallet性能而准备了benchmarking框架。
> - 实现了供核心客户端调用运行时的接口。
>
> 在[构造](https://docs.substrate.io/build/)和[测试](https://docs.substrate.io/test/)中有更多关于运行时的构建、定义基准测试、使用运行时的接口等话题的知识。现在对这些内容有个大概了解就成。

关于SNT的结构划分，可以参考一下[旧版仓库](https://github.com/Endericedragon/substrate-node-template-copy?tab=readme-ov-file#template-structure)里的描述。简单翻译一下：

> 像这样的Substrate项目通常都包含许多组件，它们分布在各个不同的目录中。
>
> **Node组件**
>
> 一个区块链节点，就是一个允许用户参与到一个区块链网络中的应用程序。基于Substrate开发的区块链节点，提供了许多功能：
>
> - 网络：Substrate节点使用[libp2p](https://libp2p.io)网络协议栈（译者注：这个协议栈有官方认可的[rust实现](https://github.com/libp2p/rust-libp2p)），实现了区块链网络中节点之间的通信。
> - 共识：[共识算法](https://docs.substrate.io/learn/consensus/)是区块链必须的，这样才能确定网络的状态。Substrate允许开发者提供自定义的共识引擎，同时也内置了多个基于Web3基金会研究的共识机制。
> - RPC服务器：用于与Substrate节点进行交互的远程过程调用（RPC）服务器。
>
> `node`目录中有许多文件，其中有几个值得特别留意：
>
> - `chain_spec.rs`：规定了Substrate链的初始状态（创世状态genesis state）。这玩意在开发和测试的时候很好使，而且在生产环境中也很重要。其中的`development_config`和`testnet_genesis`函数特别重要，它们定义了本地开发链的创世状态。这些函数定义了一些知名账户（例如Alice和Bob），并将它们用于配置区块链的初始状态。
> - `service.rs`：定义了Substrate节点的实现。其中的库引用和函数调用都值得关注。涉及共识相关的部分尤其值得关注，例如区块最终封装确定（finalization）和分叉等，还有其他共识机制，例如Aura用于区块产生，GRANDPA用于区块封装确定。
>
> **Runtime组件**
>
> 在Substrate的语境中，运行时（runtime）和状态转移函数（state transition function）非常类似。两者都指代区块链的核心逻辑，负责验证区块和执行状态变更。Substrate项目在本仓库中使用[FRAME](https://docs.substrate.io/learn/runtime-development/#frame)构建了区块链运行时。FRAME允许运行时开发者声明特定领域的逻辑，称为“pallet”，并将其统统组合成一个运行时，以满足各种需求。
>
> 在阅读`src/runtime/lib.rs`时，注意以下事项：
> - 该源代码文件为运行时加入了好几个pallet。每个pallet的配置都定义在一个`impl $PALLET名字::Config for Runtime`代码块中。
> - 这些pallet通过`construct_runtime!`宏组合在一起，形成了一个大的运行时。它是FRAME的核心库的一部分。
>
> **Pallets组件**
>
> 该Substrate项目的运行时除了由一大堆随同Substrate代码仓库一同发布的FRAME pallet组成，还另有一个模板pallet，位于`pallets`目录中。
>
> FRAME pallet由一大堆区块链基本构件（primitives）组成，这其中包括：
> - 存储：FRAME定义了一系列强大的存储抽象，使得Substrate的高效的键值数据库可以轻松地管理区块链的演进状态。
> - 分派函数：FRAME pallet定义了一些特殊类型的函数，可以从运行时之外的外部环境调用（dispatch）来更新其状态。
> - 事件：Substrate使用事件来通知用户发生了重大状态变化。
> - 错误：当分派函数失败时，它会返回一个错误。
>
> 每个pallet都有一个自己的`Config` trait，它作为通用接口，可以通用地定义它所需的数据类型和参数。

这样一来就指出了一条研究方向：Substrate依赖的libp2p网络协议栈，以及FRAME运行时和pallet的架构。如果能把libp2p给整到rCore上，那就能为实现真正的区块链操作系统打下很好的基础了。

## 初探Node组件

其实`node`目录底下的东西不是很复杂，才8个文件，一路看过去也应该有个了解了。从上文中我们得知这个Node组件主要干三件事：网络、共识、RPC。那么我们可以据此进行模块划分，把Node组件拆成更小的三个子模块。对`node`目录执行`cargo metadata`看看？

结果发现和对SNT根目录执行`cargo metadata`没啥差别。但是好歹`resolve`的`root`有东西了，是`path+file:///home/endericedragon/repos/substrate-node-template-copy/node#node-template@4.0.0-dev`。

在`packages`字段中搜索`node-template`，找到了`node`包，我们来看看关于它的基本信息：

```json
{
    "name": "node-template",
    "version": "4.0.0-dev",
    "id": "path+file:///home/endericedragon/repos/substrate-node-template-copy/node#node-template@4.0.0-dev",
    "license": "MIT-0",
    "license_file": null,
    "description": "A fresh FRAME-based Substrate node, ready for hacking.",
    "source": null,
    "dependencies": [...],
    "targets": [
        {
            "kind": [
                "lib"
            ],
            "crate_types": [
                "lib"
            ],
            "name": "node-template",
            "src_path": "/home/endericedragon/repos/substrate-node-template-copy/node/src/lib.rs",
            "edition": "2021",
            "doc": true,
            "doctest": true,
            "test": true
        },
        {
            "kind": [
                "bin"
            ],
            "crate_types": [
                "bin"
            ],
            "name": "node-template",
            "src_path": "/home/endericedragon/repos/substrate-node-template-copy/node/src/main.rs",
            "edition": "2021",
            "doc": true,
            "doctest": false,
            "test": true
        },
        {
            "kind": [
                "custom-build"
            ],
            "crate_types": [
                "bin"
            ],
            "name": "build-script-build",
            "src_path": "/home/endericedragon/repos/substrate-node-template-copy/node/build.rs",
            "edition": "2021",
            "doc": false,
            "doctest": false,
            "test": false
        }
    ],
    // -- snip --
}
```

