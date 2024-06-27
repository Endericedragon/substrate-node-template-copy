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

## Substrate Node Template的大致划分

首先参考一下Substrate Node Template（下简称SNT）的[旧版仓库](https://github.com/Endericedragon/substrate-node-template-copy?tab=readme-ov-file#template-structure)里的描述。简单翻译一下：

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