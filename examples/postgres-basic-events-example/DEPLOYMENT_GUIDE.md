# SUDO Raffle Indexer - Deployment Guide

## 🎉 **IMPLEMENTATION COMPLETE!**

The dynamic raffle module registration system is fully implemented and running!

---

## ✅ **What's Automated**

### **1. Instant Module Registration (HTTP Webhook)**
When you deploy a new raffle through your backend:
- Backend automatically saves it to `raffle_games` table with `is_active = true`
- Backend calls the indexer's webhook: `POST http://localhost:8086/reload-modules`
- Indexer **instantly reloads** the module list and starts monitoring the new raffle
- **No manual intervention needed!**

### **2. Fallback Mechanism (Periodic Refresh)**
- Every 60 seconds, the indexer automatically checks the database for new modules
- Even if the webhook fails, new modules are picked up within 1 minute
- Provides redundancy and reliability

### **3. Continuous Operation**
- Once started, the indexer runs continuously
- **No restart needed** when deploying new raffles
- Dynamically updates its monitoring list on the fly

---

## 🚀 **Current Status**

### **Indexer is Running**
- **Process Manager**: PM2 (ID: 33)
- **Status**: ✅ Online
- **Auto-restart**: Enabled
- **HTTP Server**: http://localhost:8086
  - Health check: `GET /health`
  - Reload endpoint: `POST /reload-modules`

### **Monitoring**
- **32 active raffle modules** loaded
- **Connected to**: Aptos Devnet (Chain ID: 207)
- **Starting version**: 9813874
- **Current version**: ~9982480+

---

## 📋 **PM2 Commands**

### **Check Status**
```bash
pm2 status sudo-raffle-indexer
```

### **View Logs**
```bash
# Real-time logs
pm2 logs sudo-raffle-indexer

# Last 100 lines
pm2 logs sudo-raffle-indexer --lines 100 --nostream

# Error logs only
pm2 logs sudo-raffle-indexer --err
```

### **Restart/Stop/Start**
```bash
pm2 restart sudo-raffle-indexer
pm2 stop sudo-raffle-indexer
pm2 start sudo-raffle-indexer
```

### **Monitor Resources**
```bash
pm2 monit
```

---

## 🔧 **Configuration Files**

### **1. Indexer Config** (`config.yaml`)
```yaml
health_check_port: 8085
server_config:
  transaction_stream_config:
    indexer_grpc_data_service_address: "https://grpc.devnet.aptoslabs.com:443"
    auth_token: "aptoslabs_S2oG3GwzNWK_PBcesGHPStRQBuqoYbp2a72AWEtppwCMx"
    request_name_header: "events-processor"
    starting_version: 9813874
  postgres_config:
    connection_string: postgresql://postgres:postgres@localhost:5432/sudo_indexer
```

### **2. PM2 Ecosystem** (`ecosystem.config.js`)
```javascript
module.exports = {
  apps: [{
    name: 'sudo-raffle-indexer',
    cwd: '/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example',
    script: 'cargo',
    args: 'run --release -- --config-path config.yaml',
    interpreter: 'none',
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: '2G',
    env: {
      DATABASE_URL: 'postgresql://postgres:postgres@localhost:5432/sudo_indexer',
      RUST_LOG: 'info',
      RUST_BACKTRACE: '1'
    }
  }]
};
```

---

## 🧪 **Testing the System**

### **1. Test Health Endpoint**
```bash
curl http://localhost:8086/health
# Expected: "Indexer is running"
```

### **2. Test Manual Reload**
```bash
curl -X POST http://localhost:8086/reload-modules \
  -H "Content-Type: application/json" \
  -d '{"module_address": "0xYOUR_MODULE_ADDRESS"}'
```

### **3. Deploy a New Raffle**
1. Deploy a new raffle through your frontend/backend
2. Watch the indexer logs: `pm2 logs sudo-raffle-indexer`
3. You should see: "✅ Successfully reloaded modules: X active"
4. The new raffle events will be indexed automatically

### **4. Check Database**
```bash
PGPASSWORD=postgres psql -U postgres -h localhost -d sudo_indexer \
  -c "SELECT COUNT(*) FROM buy_events;"

PGPASSWORD=postgres psql -U postgres -h localhost -d sudo_indexer \
  -c "SELECT COUNT(*) FROM raffle_events;"
```

---

## 🔄 **How It Works**

### **Data Flow**
```
1. User deploys new raffle
   ↓
2. Backend saves to raffle_games table (is_active=true)
   ↓
3. Backend calls: POST http://localhost:8086/reload-modules
   ↓
4. Indexer reloads modules from database
   ↓
5. Indexer starts monitoring new module's events
   ↓
6. Events are indexed to buy_events & raffle_events tables
   ↓
7. Backend queries events and sends to frontend via Socket.IO
   ↓
8. Frontend displays in Raffle History & Leaderboard
```

### **Module Loading**
- **On startup**: Loads all active modules from `raffle_games` table
- **Every 60s**: Automatic refresh from database
- **On webhook call**: Instant reload when backend notifies

---

## 🐛 **Troubleshooting**

### **Indexer Not Running**
```bash
# Check status
pm2 status sudo-raffle-indexer

# If stopped, start it
pm2 start sudo-raffle-indexer

# Check logs for errors
pm2 logs sudo-raffle-indexer --err
```

### **Chain ID Error**
If you see "Wrong chain id detected", the devnet was reset:
```bash
# Clear old chain data
PGPASSWORD=postgres psql -U postgres -h localhost -d sudo_indexer \
  -c "DELETE FROM processor_metadata.ledger_infos; 
      DELETE FROM processor_metadata.processor_status;
      TRUNCATE TABLE buy_events, raffle_events CASCADE;"

# Restart indexer
pm2 restart sudo-raffle-indexer
```

### **Webhook Not Working**
```bash
# Test manually
curl -X POST http://localhost:8086/reload-modules \
  -H "Content-Type: application/json" \
  -d '{"module_address": "test"}'

# Check if port 8086 is open
netstat -tlnp | grep 8086
```

### **No Events Being Indexed**
1. Check if module is in the active list:
   ```bash
   pm2 logs sudo-raffle-indexer | grep "Updated active modules"
   ```
2. Verify module address format in database
3. Check if events are being emitted on-chain
4. Verify GRPC connection is active

---

## 📊 **Monitoring**

### **Key Metrics to Watch**
- **PM2 Status**: Should always be "online"
- **Restart Count** (`↺`): Should be low (< 10)
- **Memory Usage**: Should stay under 2GB
- **Log Errors**: Should be minimal

### **Log Files**
- **Output**: `/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/logs/indexer-out.log`
- **Errors**: `/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/logs/indexer-error.log`

---

## 🎯 **Next Steps**

### **For Production**
1. **Set up systemd** (optional, PM2 is already good):
   ```bash
   pm2 startup
   # Follow the instructions
   ```

2. **Configure log rotation**:
   ```bash
   pm2 install pm2-logrotate
   pm2 set pm2-logrotate:max_size 100M
   pm2 set pm2-logrotate:retain 7
   ```

3. **Set up monitoring**:
   - PM2 Plus (paid): https://pm2.io/
   - Or use your own monitoring solution

4. **Backup strategy**:
   - Regular database backups
   - Keep PM2 dump file: `/root/.pm2/dump.pm2`

---

## 📝 **Summary**

**You DON'T need to manually run `cargo run` anymore!**

The system is fully automated:
- ✅ Indexer runs continuously via PM2
- ✅ Auto-restarts on crashes
- ✅ Auto-starts on server reboot (if PM2 startup is configured)
- ✅ Dynamically loads new raffle modules
- ✅ Instant updates via webhook
- ✅ Fallback periodic refresh every 60s

**Just deploy your raffles and they'll be automatically indexed!** 🎉

