# pricing_daemon
Daemon process that maintain __BTCUSD__ pricing data by fetching from Kraken and store in __Mongodb Atlas__.

# Usage
```
API_KEY=<API_KEY> \
API_SECRET=<API_SECRET> \
DB_USERNAME=<DB_USERNAME> \
DB_PASSWORD=<DB_PASSWORD> \
DB_HOST=<DB_HOST> \
DB_NAME= \
RUST_LOG_STYLE=auto RUST_LOG=pricing_daemon=info,warn,error cargo run
```
