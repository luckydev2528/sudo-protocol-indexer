1. You shouldn’t run cargo run in production

cargo run is intended for development. It recompiles every time you restart, which is slow and fragile.
Instead, you should build the binary once and then run that binary under pm2.

# Build release binary
cargo build -p postgres-basic-events-example --release

# The binary will be here:
target/release/postgres-basic-events-example

Now you can run it directly:
./target/release/postgres-basic-events-example -c postgres-basic-events-example/config.yaml

2. Running with pm2

PM2 is designed for Node.js, but it also works fine for any executable (via --interpreter none).

Example:
pm2 start ./target/release/postgres-basic-events-example \
  --name aptos-indexer \
  --interpreter none \
  -- -c postgres-basic-events-example/config.yaml

--interpreter none tells pm2 this is not a Node app.
Everything after -- is passed as arguments to your binary.
--name gives your process a friendly name for monitoring.

3. Managing the process
# Check logs
pm2 logs aptos-indexer

# Restart
pm2 restart aptos-indexer

# Auto-start on reboot
pm2 startup
pm2 save
