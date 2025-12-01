# API Initialization Issue

**Date**: November 30, 2025
**Status**: Critical - API fails to start

## Problem: API Silent Failure on Startup

**Severity**: CRITICAL

**Description**:
The Intent Segregation API (`intent-api` binary) process exits silently within 1-3 seconds without binding to port 8080 or printing any error messages.

**How to Reproduce**:
```bash
# Option 1: Run binary directly
target\debug\intent-api.exe
# Exits immediately with no output

# Option 2: Run via cargo
cargo run --bin intent-api
# Exits after 2-3 seconds, no output

# Verify failure
curl http://127.0.0.1:8080/health
# Connection refused
```

**What Should Happen**:
- Process should print startup messages
- HTTP server should bind to port 8080
- `curl http://127.0.0.1:8080/health` should return JSON
- Process should run until Ctrl+C

**What Actually Happens**:
- Process exits silently
- No error messages printed
- No output to console
- Port 8080 unbound
- Database connection likely fails

**Possible Root Causes**:
1. **Database connection fails** - PostgreSQL pool initialization fails silently
2. **Config not loading** - Invalid database URL or credentials in `config/default.toml`
3. **Port binding fails** - Port 3000 or 8080 already in use
4. **Middleware initialization fails** - CORS or logging setup errors

**Evidence**:
- PostgreSQL container verified running: `docker ps` shows `intent-postgres`
- Database responds to direct queries: `docker exec intent-postgres psql -U intent_user -d intent_segregation -c "SELECT 1;"` returns `1`
- Code compiles successfully with no errors
- Binary exists at `target/debug/intent-api.exe`
- No error messages are printed to console

## Fix Required

Add startup logging to `api/src/main.rs` to identify where initialization fails:

```rust
// After loading config
eprintln!("[STARTUP] Configuration loaded");
eprintln!("[STARTUP] Database URL: {}", masked_url);

// Before database connection
eprintln!("[STARTUP] Creating database connection pool...");

// After database connection
eprintln!("[STARTUP] Database pool created successfully");

// Before binding listener
eprintln!("[STARTUP] Binding to 0.0.0.0:8080");

// After binding listener
eprintln!("[STARTUP] Server listening on 0.0.0.0:8080");

// Add explicit error printing
.await
.map_err(|e| eprintln!("[FATAL] {}", e))?
```

This will reveal which initialization step is failing and why the process exits silently.

