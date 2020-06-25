# cosmwasm-simulate
Simulation tool of Cosmwasm smart contract

# Overview
cosmwasm-simulate is developed for Cosmwasm Smart Contract system, the main functions is:
* Fast load&deploy contract without run WASMD
* Fast call contract interface via command
* Print some debug information on screen
* Do some bytecode check during wasm instanced
* Watching storage db change on realtime
* Dynamic calcuate and printing gas used during contract execute 
* Easy to test smart contract without input a json string
# Build
```shell script
cargo +nightly build
```
# Guide
## Simulate deploy
* Download wasm file
```
cd ~/github.com/cosmwasm/cosmwasm-examples/erc20/
wget https://github.com/CosmWasm/cosmwasm-examples/raw/master/erc20/contract.wasm
```

* Run cosmwasm-simulate like:
```shell script
cosmwasm-simulate ~/github.com/cosmwasm/cosmwasm-examples/erc20/contract.wasm
```
* Command like follow:
```shell script
cosmwasm-simulate [wasm_file]
```
##### Attention: You must make sure that must include directory: [schema](https://github.com/CosmWasm/cosmwasm-examples/tree/master/erc20/schema) at same directory of`wasm_file`

## Simulate run
cosmwasm-simulate will auto load json schema file to analyze all message type and structure type after code compile complete.   
it will guide you to enter the correct command and data structure

### Example
For example,we use repo`~/github.com/cosmwasm/cosmwasm-examples/erc20/contract.wasm` to test this tool，you can download erc20 contract example from [Cosmwasm-github](https://github.com/CosmWasm/cosmwasm-examples)   
1 .Load wasm   
```shell script
cosmwasm-simulate ~/github.com/cosmwasm/cosmwasm-examples/erc20/contract.wasm
```
2 .Input `init`   
```shell script
Input call type(init | handle | query):
init
```
3 .Input Message type name`InitMsg` which will print out on screen
```shell script   
Input Call param from [ Constants | HandleMsg | QueryMsg | InitMsg | BalanceResponse | AllowanceResponse |  ]
InitMsg
InitMsg {
	decimals : integer
	initial_balances : InitialBalance :{
		address : HumanAddr
		amount : Uint128
	}
	name : string
	symbol : string
}
```
4 .Input every member of InigMsg step by step
```shell script
input [decimals]:
9
input [initial_balances]:
input 	[address : HumanAddr]:
ADDR0012345
input 	[amount : Uint128]:
112233445
input [name]:
OKB
input [symbol]:
OKBT
```
5 .Finish init  
The tool will print DB Changes and Gas used on screen
```shell script
***************************call started***************************
executing func [init] , params is {"decimals":9,"initial_balances":[{"address":"ADDR0012345","amount":"112233445"}],"name":"OKB","symbol":"OKBT"}
DB Changed : [Insert]
Key        : balancesADDR0012345]
Value      : [000862616c616e6365734144445230303132333435000000000000000000]
DB Changed : [Insert]
Key        : [configconstants]
Value      : [{"name":"OKB","symbol":"OKBT","decimals":9}]
DB Changed : [Insert]
Key        : [configtotal_supply]
Value      : [0006636f6e666967746f74616c5f737570706c79]
init msg.data: =
Gas used   : 59422
***************************call finished***************************
Call return msg [Execute Success]
```
6 .call query   
```shell script
Input call type(init | handle | query):
query
Input Call param from [ Constants | HandleMsg | QueryMsg | InitMsg | BalanceResponse | AllowanceResponse |  ]
QueryMsg
Input Call param from [ allowance | balance |  ]
balance
```
7 .Input every member of QueryMsg step by step
```shell script
input [address]:
ADDR0012345
JsonMsg:{"balance":{"address":"ADDR0012345"}}
***************************call started***************************
executing func [query] , params is {"balance":{"address":"ADDR0012345"}}
query msg.data: = {"balance":"112233445"}
Gas used   : 19239
***************************call finished***************************
Call return msg [Execute Success]
```
# Future
* More customization function
* Make cosmwasm-simulate visualization `(html+js+rpc)`
* Upgrade and sync with cosmwasm
* More features support
