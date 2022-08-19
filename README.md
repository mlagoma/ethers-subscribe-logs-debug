# Debug Rust ethers subscribe_logs

Bare minimum hardhat (typescript) and ethers (Rust) apps to demonstrate issue with ethers websocket subscribe_logs getting stuck.

## Prerequisites

Node and Rust

## Install and compile hardhat contract:

Change directory to basic_hardhat

```shell
npm install
npm run compile
```

## Run hardhat node:

Change directory to basic_hardhat

```shell
npm run node-start
```

## Install and run ethers subscribe_logs:

Change directory to basic_subscribe_logs

```shell
cargo run
```

## Trigger an event

Change directory to basic_hardhat

```shell
npm run event
```

## Reproduce the issue

1. Start hardhat node and ethers subscribe_logs in separate terminals
2. In another terminal, trigger an event a few times to see events logged in subscribe_logs.
2. Wait for a while (5-10 minutes typically).
3. Trigger an event again and observe no new log.
4. Restart subscribe_logs to see the new log picked up after restart.