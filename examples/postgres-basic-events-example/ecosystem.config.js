module.exports = {
  apps: [
    {
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
      },
      error_file: '/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/logs/indexer-error.log',
      out_file: '/home/sudo/sudo-protocol-indexer/examples/postgres-basic-events-example/logs/indexer-out.log',
      log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
      merge_logs: true,
      time: true
    }
  ]
};

