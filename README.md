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
* Run cosmwasm-simulate:
```shell script
cosmwasm-simulate -m messages.json
```
* messages.json content:
```json
{
  "messages": [
      {
        "wasm_file": "erc20/erc20.wasm",
        "type": "init", "contract_addr": "contract_addr1", "sender": "s1",
        "message": {"name":"Test eth token","symbol":"eth","decimals":10,"initial_balances":[{"address":"account_addr1","amount":"10066"}]}
      },
      {
        "wasm_file": "erc21/erc20.wasm",
        "type": "init", "contract_addr": "contract_addr2", "sender": "s1",
        "message": {"name":"Test btc token","symbol":"btc","decimals":10,"initial_balances":[{"address":"account_addr2","amount":"10067"}]}
      },
  
      {
        "type": "query", "contract_addr": "contract_addr1",
        "message": {"balance":{"address":"account_addr1"}}
      },
      {
        "type": "query", "contract_addr": "contract_addr2",
        "message": {"balance":{"address":"account_addr2"}}
      }
    ]
}
```

## Simulate run

1. Start up simulate
```shell script
cosmwasm-simulate -m messages.json
```

2. Output `init`   
```shell script
load messages from: msg.json
Compiling [erc20/erc20.wasm]...
successfully loaded [erc20/erc20.wasm]
Compiling [erc21/erc20.wasm]...
successfully loaded [erc21/erc20.wasm]
***************************call started***************************
init: contract address[contract_addr1], sender[s1], params[{"decimals":10,"initial_balances":[{"address":"account_addr1","amount":"10066"}],"name":"Test okchain token","symbol":"OKT"}]
DB Changed : [Insert]
Key        : [ balancesaccount_addr1       ]
Value      : [              'R]
DB Changed : [Insert]
Key        : [ configconstants]
Value      : [{"name":"Test okchain token","symbol":"OKT","decimals":10}]
DB Changed : [Insert]
Key        : [ configtotal_supply]
Value      : [              'R]
init msg.data: = 
Gas used   : 61390
***************************call finished***************************
***************************call started***************************
init: contract address[contract_addr2], sender[s1], params[{"decimals":10,"initial_balances":[{"address":"account_addr2","amount":"10067"}],"name":"Test okchain token","symbol":"OKT"}]
DB Changed : [Insert]
Key        : [ balancesaccount_addr2       ]
Value      : [              'S]
DB Changed : [Insert]
Key        : [ configconstants]
Value      : [{"name":"Test okchain token","symbol":"OKT","decimals":10}]
DB Changed : [Insert]
Key        : [ configtotal_supply]
Value      : [              'S]
init msg.data: = 
Gas used   : 61390
***************************call finished***************************
***************************call started***************************
query: contract address[contract_addr1], sender[], params[{"balance":{"address":"account_addr1"}}]
query msg.data: = {"balance":"10066"}
Gas used   : 18023
***************************call finished***************************
***************************call started***************************
query: contract address[contract_addr2], sender[], params[{"balance":{"address":"account_addr2"}}]
query msg.data: = {"balance":"10067"}
Gas used   : 18023
***************************call finished***************************
```

# Future
* More customization function
* Make cosmwasm-simulate visualization `(html+js+rpc)`
* Upgrade and sync with cosmwasm
* More features support

