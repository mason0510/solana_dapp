# SPL token

`SPL`的token类比eth上的erc20代币，solana将这些常用的协议自己实现了，可以通过命令行的形式交互

### 安装
```shell
cargo install spl-token-cli
spl-token --version
##spl-token-cli 2.0.15
```
### test
```shell
## 发行
proxychains4 spl-token create-token
返回
#Creating token 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf
#Signature: 2XMXWVqSKYNooncFd2mX2LFT3U7S9cbn9tmfAiKEtyeakMtVZ9xbUGjfFpiXFYW3CCXDZmgtTmTQpQysX4vqGDmo

## 查询供应量
proxychains4 spl-token supply 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf
# 0

## 在该token中创建账户
proxychains4 spl-token create-account  7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf
##Creating account 9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW
##Signature: aY92N3vUiXRK5DrcpFibmygu8RuQujoEtEgxpbKWwWviAX2UPvbd2Qo1N49koHFhthAvZ6GF93uQK8F6ZYrgASL

##
proxychains4 spl-token balance 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf 
## 100

## mint
proxychains4 spl-token mint 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf 100
# Minting 100 tokens
#  Token: 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf
#  Recipient: 9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW
# Signature: 5sPMhmfKy8zCEUgi4rkYnVXcnJNSptnVoneJnvH8SnzLLQ5BkQBSRDxpPsNyqH5RfxV166M88Ye1WQnv9eRMyKvz

## trransfer
proxychains4 spl-token transfer 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf 50 5RQUBrq2aDfLnjVAakgQi4NCbMuQUJULj2Xfe8appgU1
#Transfer 50 tokens
#  Sender: 9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW
#  Recipient: 5RQUBrq2aDfLnjVAakgQi4NCbMuQUJULj2Xfe8appgU1
#  Recipient associated token account: HXa2YpGhS8GQUY2nsbkQ5VJDm5rxZQ4wxF1BsT28FJjt
# Signature: 21GGo4c9BYF1c599Yhpj4nhf9SbV1o4yBiE9Wwh1j12Wr4sBnYkhr5MSCJdh2Z5TSWFzRnDciCU9hZ9pqryLQo7f


```
