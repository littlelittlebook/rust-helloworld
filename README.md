# 

The contract code and the build setup are derived from:
https://docs.near.org/docs/develop/contracts/rust/intro


#### contract code: 
src/lib.rs

#### build setup
see cargo.toml


#### run test:
`cargo test -- --nocapture`

#### build release:
`cargo build --target wasm32-unknown-unknown --release`

#### deploy
```
export ID=example-helloworld.littlebook.testnet
near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_helloworld.wasm --accountId $ID
```

#### call helloworld()
`near call $ID helloworld "{\"name\":\"bob\"}" --accountId $ID`

example output

```
$ near call $ID helloworld "{\"name\":\"bob\"}" --accountId $ID
Scheduling a call: example-helloworld.littlebook.testnet.helloworld({"name":"bob"})
Doing account.functionCall()
Transaction Id CL3bT6LqaHP49rXCXCaqyp8SUDr8N4giEjeEysmYRVJf
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/CL3bT6LqaHP49rXCXCaqyp8SUDr8N4giEjeEysmYRVJf
'hello world! bob'
```

