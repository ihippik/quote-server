# 📡 quote-server

A lightweight experimental project for streaming **fake stock quotes** using **TCP + UDP**.  
Perfect for learning Rust networking, concurrency, thread synchronization, and building TCP/UDP-based protocols.

This repository includes:

- 🖥️ **Quote Server** — handles TCP subscriptions and pushes filtered quotes via UDP
- 💻 **CLI Client** — subscribes to tickers, receives live quotes over UDP, and keeps the stream alive with ping messages

---

## 🚀 Overview

### How it works

1. **Server**
    - Generated random quotes for tickers from the ticket file
    - Accepts TCP connections from clients
    - Parses `STREAM` commands
    - Registers per-client quote channels
    - Streams filtered `StockQuote` messages over UDP
    - Removes dead clients automatically

2. **Client**
    - Connects via TCP
    - Requests a quote stream for selected tickers
    - Listens on a UDP port for live quotes
    - Sends periodic `PING` UDP messages to keep the stream active
    - Handles graceful shutdown on `Ctrl+C`

### Examples

### ▶️ Running the server
```bash
run --package quote-server --bin server -- --address 127.0.0.1:8084
```

### ▶️ Running the client
```bash
run --package quote-server --bin client -- --address 127.0.0.1:8084 --udp_port 34255 --tickets AAPL,TSLA
```