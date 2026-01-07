# SPL_Token_2022

本项目提供Rust工具，用于在Solana区块链上使用Token 2022程序创建和管理SPL代币。

## 功能

- 创建带有元数据的SPL新代币
- 初始化代币账户（ATA）
- 铸造代币
- 撤销给定代币地址的各种权限（冻结、铸造、元数据更新、元数据指针）

## 使用方法

1. 在`.env`文件中设置所需的环境变量：
   - `RPC_URL`：Solana RPC URL
   - `WALLET_PATH`：Solana钱包文件路径
   - `METADATA_URI`：代币元数据的URI（创建新代币时需要）
   - `TOKEN_ADDRESS`：要撤销权限的代币地址（撤销权限时需要）

2. 运行`cargo run --bin create_token`创建新的SPL代币。
3. 运行`cargo run --bin revoke_auth`撤销现有代币的权限。

## 依赖

- `solana-client`
- `solana-sdk`
- `spl-token-2022-interface`
- `spl-token-metadata-interface`
- `dotenv`

## 备注

本项目使用Solana Token 2022程序创建和管理代币。包括创建带有元数据的新代币和撤销各种权限的示例。