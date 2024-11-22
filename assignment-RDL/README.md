# Financial Data Fetcher in Rust

## Overview
This Rust application periodically fetches and records the pricing data of:
- Bitcoin
- Ethereum
- S&P 500 Index

## Features
- Fetches real-time pricing data from [CoinGecko API](https://www.coingecko.com/).
- Saves pricing data to separate text files.
- Runs every 10 seconds indefinitely.