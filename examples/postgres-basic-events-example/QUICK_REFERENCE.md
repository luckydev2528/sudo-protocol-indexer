# Quick Reference - SUDO Raffle Indexer

## ✅ **System Status**

**The indexer is running and fully automated!**

---

## 🚀 **Quick Commands**

### **Check if Running**
```bash
pm2 status sudo-raffle-indexer
curl http://localhost:8086/health
```

### **View Logs**
```bash
pm2 logs sudo-raffle-indexer
```

### **Restart**
```bash
pm2 restart sudo-raffle-indexer
```

---

## 🎯 **How It Works**

1. **You deploy a new raffle** → Backend saves to database
2. **Backend automatically notifies indexer** → Instant reload
3. **Indexer starts monitoring** → Events are indexed
4. **Frontend displays data** → Raffle History & Leaderboard populated

**No manual intervention needed!**

---

## 📊 **What's Automated**

- ✅ **Instant module registration** via HTTP webhook
- ✅ **Periodic refresh** every 60 seconds (fallback)
- ✅ **Auto-restart** on crashes (PM2)
- ✅ **Continuous monitoring** of all active raffles

---

## 🔧 **Endpoints**

- **Health**: `GET http://localhost:8086/health`
- **Reload**: `POST http://localhost:8086/reload-modules`

---

## 📝 **Key Files**

- **Config**: `config.yaml`
- **PM2 Config**: `ecosystem.config.js`
- **Logs**: `logs/indexer-out.log`, `logs/indexer-error.log`
- **Full Guide**: `DEPLOYMENT_GUIDE.md`

---

## ⚠️ **If Something Goes Wrong**

```bash
# 1. Check status
pm2 status sudo-raffle-indexer

# 2. Check logs
pm2 logs sudo-raffle-indexer --err

# 3. Restart
pm2 restart sudo-raffle-indexer

# 4. If chain ID error (devnet reset):
PGPASSWORD=postgres psql -U postgres -h localhost -d sudo_indexer \
  -c "DELETE FROM processor_metadata.ledger_infos; 
      DELETE FROM processor_metadata.processor_status;
      TRUNCATE TABLE buy_events, raffle_events CASCADE;"
pm2 restart sudo-raffle-indexer
```

---

## 🎉 **That's It!**

The system is fully operational. Just deploy your raffles and they'll be automatically indexed!

For detailed information, see `DEPLOYMENT_GUIDE.md`

