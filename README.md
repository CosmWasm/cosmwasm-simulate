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
```
cargo +nightly build
```
# Guide
## Simulate
* Run cosmwasm-simulate:
```
cosmwasm-simulate -m messages.json
```
* messages.json content:
```
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

* Output   
```
load messages from: msg.json
Compiling [erc20/erc20.wasm]...
successfully loaded [erc20/erc20.wasm]
Compiling [erc21/erc20.wasm]...
successfully loaded [erc21/erc20.wasm]
***************************call started***************************
init: contract address[contract_addr1], sender[s1], params[{"decimals":10,"initial_balances":[{"address":"account_addr1","amount":"10066"}],"name":"Test eth token","symbol":"eth"}]
DB Changed : [Insert]
Key        : balancesADDR0012345]
Value      : [000862616c616e6365734144445230303132333435000000000000000000]
DB Changed : [Insert]
Key        : [configconstants]
Value      : [{"name":"eth","symbol":"eth","decimals":9}]
DB Changed : [Insert]
Key        : [configtotal_supply]
Value      : [0006636f6e666967746f74616c5f737570706c79]
init msg.data: =
init msg.data: = 
Gas used   : 61390
***************************call finished***************************
***************************call started***************************
init: contract address[contract_addr2], sender[s1], params[{"decimals":10,"initial_balances":[{"address":"account_addr2","amount":"10067"}],"name":"Test btc token","symbol":"btc"}]
DB Changed : [Insert]
Key        : balancesADDR0012345]
Value      : [000862616c616e6365734144445230303132333435000000000000000000]
DB Changed : [Insert]
Key        : [configconstants]
Value      : [{"name":"btc","symbol":"BTC","decimals":9}]
DB Changed : [Insert]
Key        : [configtotal_supply]
Value      : [0006636f6e666967746f74616c5f737570706c79]
init msg.data: =
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

