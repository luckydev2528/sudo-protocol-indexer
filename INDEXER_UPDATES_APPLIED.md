# ✅ Indexer Updates Applied for Devnet FA Raffle

## Summary
Updated the indexer to work with the new Devnet FA Raffle deployment at `0x38804aa2186ed8e3a33e5c3dc071d8d0248f578dd7fa0bff382b72df85b8a22f`.

⚠️ **IMPORTANT**: The Devnet gRPC indexer endpoint requires a valid auth token. You need to get one from [Geomi (Aptos Build)](https://developers.aptoslabs.com/docs/introduction) before running the indexer.

---

## Files Modified

### 1. config.yaml
**Path:** `/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/config.yaml`

**Changes:**
- Network: `https://grpc.testnet.aptoslabs.com:443` → `https://grpc.devnet.aptoslabs.com:443`
- Starting version: `6694522045` → `431944650` (first Devnet deployment)

### 2. raffle_events_model.rs
**Path:** `/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/src/raffle_events_model.rs`

**Changes:**
- Event filter: Changed from multiple testnet addresses with `::meme::RaffleEvent` to single Devnet address with `::fa_raffle::RaffleEvent`
- Module address: → `0x38804aa2186ed8e3a33e5c3dc071d8d0248f578dd7fa0bff382b72df85b8a22f`
- Event struct: `coin_type: String` → `fa_metadata: FaMetadata`
- Added `FaMetadata` struct with `inner: String` field
- Field access: `data.coin_type` → `data.fa_metadata.inner`

### 3. buy_events_model.rs
**Path:** `/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/src/buy_events_model.rs`

**Changes:**
- Event filter: Changed from multiple testnet addresses with `::meme::BuyEvent` to single Devnet address with `::fa_raffle::BuyEvent`
- Module address: → `0x38804aa2186ed8e3a33e5c3dc071d8d0248f578dd7fa0bff382b72df85b8a22f`
- Event struct: `coin_type: String` → `fa_metadata: FaMetadata`
- Added `FaMetadata` struct with `inner: String` field
- Field access: `data.coin_type` → `data.fa_metadata.inner`

---

## Key Changes Summary

### Network
- **Testnet** → **Devnet**
- gRPC endpoint now points to Devnet

### Module Addresses
**Old (Testnet, multiple addresses):**
- `0xb359560fc77127a3d1ccdad4bd7f8423997a16a3572dcea4e4dd747fd3ecd08b`
- `0xc58654a8eaa2818496ab82ae67dbae67963cd429267da4db1039de0bb712ef07`
- `0xd7726cb41ba5b84c22af0038066baab6cf892ef9191cce0a6137f4642d57be5e`
- `0x1f9fce92ec5b8ef68d4f1925269cec38dda5a7855a80d056e0d39ca3f3682f18`

**New (Devnet, single address):**
- `0x38804aa2186ed8e3a33e5c3dc071d8d0248f578dd7fa0bff382b72df85b8a22f`

### Module Names
- `::meme::RaffleEvent` → `::fa_raffle::RaffleEvent`
- `::meme::BuyEvent` → `::fa_raffle::BuyEvent`

### Event Data Structure
- **Old:** Coin standard with `coin_type` field
- **New:** FA standard with `fa_metadata: { inner: "0x..." }` object

---

## Running the Indexer

### Prerequisites
1. PostgreSQL running on `localhost:5432`
2. Database `sudo_indexer` created
3. Rust toolchain installed
4. **Valid Aptos Indexer API Key** (Get from [Geomi](https://developers.aptoslabs.com/))

### Getting an Auth Token
1. Visit [Geomi (Aptos Build)](https://developers.aptoslabs.com/)
2. Sign up / Log in
3. Navigate to "API Keys" section
4. Generate a new API key for Indexer access
5. Copy the auth token and update `config.yaml`:
   ```yaml
   auth_token: "your_token_here"
   ```

### Compile
```bash
cd /home/sudo/sudo-protocol-indexer/examples
cargo build --release
```

### Run
```bash
cd /home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example
cargo run --release -- --config-path config.yaml
```

### Verify Events
Connect to the database and query:
```sql
-- Check raffle events
SELECT * FROM raffle_events ORDER BY transaction_version DESC LIMIT 10;

-- Check buy events
SELECT * FROM buy_events ORDER BY transaction_version DESC LIMIT 10;
```

---

## Event Types Indexed

### RaffleEvent
- `fa_metadata` (Object<Metadata>)
- `sequence` (u64)
- `winner` (address)
- `total_tickets` (u64)
- `amount_apt` (u64)
- `amount_token` (u64)
- `timestamp` (u64)

### BuyEvent
- `fa_metadata` (Object<Metadata>)
- `sequence` (u64)
- `buyer` (address)
- `amount_apt` (u64)
- `timestamp` (u64)

---

## Related Files
- Main deployment summary: `/home/sudo/aptos-index-protocol/move/TEST_SUMMARY.md`
- Frontend/backend updates: `/home/sudo/aptos-index-protocol/UPDATES_APPLIED.md` 