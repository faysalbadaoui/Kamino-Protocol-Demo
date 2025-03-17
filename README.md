# Kamino Protocol Demo

This repository provides a demonstration of the Kamino Protocol, focusing on deposit and borrow functionalities within the Solana blockchain ecosystem.

## Overview

Kamino is a decentralized finance (DeFi) protocol on the Solana blockchain, offering a suite of products that integrate various DeFi primitives into sophisticated strategies, all wrapped in a user-friendly interface. ([app.kamino.finance](https://app.kamino.finance/?utm_source=chatgpt.com))

## Repository Structure

The repository is organized as follows:

- **client/**: Contains the front-end application built with TypeScript, facilitating user interactions with the protocol.
- **migrations/**: Includes scripts for deploying and upgrading the Solana programs.
- **programs/kamino-demo/**: Houses the main Rust-based on-chain program implementing the protocol's core logic.
- **tests/**: Comprises test scripts to validate the functionality of the protocol.

## Getting Started

To set up the project locally, follow these steps:

1. **Clone the repository**:

   ```bash
   git clone https://github.com/faysalbadaoui/Kamino-Protocol-Demo.git
   cd Kamino-Protocol-Demo
   ```

 2. **Install dependencies:**
    ```bash
    cd programs/kamino-demo
    cargo build
    ```
 3. **Deploy the Solana program:**
    ```bash
     solana program deploy ./target/deploy/kamino_demo.so
    ```

## Features

- Deposit Functionality: Allows users to deposit assets into the protocol, receiving yield-bearing tokens in return.
- Borrow Functionality: Enables users to borrow assets against their deposits, facilitating leveraged strategies.

## Development Notes

- This implementation was done without developer documentation from Kamino. All functionalities are built based on the IDL (Interface Definition Language) of the Kamino protocol found in the official docs.
- Tests have been written but will not work if the main functions are not validated to operate correctly.
