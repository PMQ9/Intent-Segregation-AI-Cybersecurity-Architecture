# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ordo Maledictum Promptorum - A Rust-based security system designed to prevent prompt injection attacks by separating user intent from user content. The system treats all inputs as potentially corrupted, tests them using sacrificial AI sentries (The Penitent Cogitators), and uses multiple independent parsers with consensus voting to validate intents before execution.

**Core Security Principle**: Never allow unvalidated user content to directly influence system behavior. All user inputs are treated with zero trust, tested on isolated models, parsed into structured intents, validated through multiple layers, and executed via typed function calls only.

## Change Documentation

**IMPORTANT - Review Before Committing**: Always review all changes before committing. Changes should be staged, reviewed for correctness, and verified to compile/test before being committed to the repository.

**IMPORTANT - Update Changelog**: Whenever you make changes to this codebase, update [CHANGELOG.md](CHANGELOG.md) with a short summary of your changes. Follow the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format and categorize changes as:
- **Added**: New features or files
- **Changed**: Modifications to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Deleted features or files
- **Fixed**: Bug fixes
- **Security**: Security-related changes

## Build & Run Commands

### Building
```bash
# Build all workspace members
cargo build

# Build optimized release version
cargo build --release

# Build specific package
cargo build -p intent-api
cargo build -p intent-parsers

# Clean build artifacts
cargo clean
```

### Running
```bash
# Quick start (runs API + frontend + checks services)
./run_local.sh

# Run API server manually
cargo run --bin intent-api

# Run with hot-reload
cargo install cargo-watch
cargo watch -x run

# Frontend dev server (separate terminal)
cd frontend && npm run dev
```

### Testing
```bash
# Run all tests
cargo test

# Test specific package
cargo test -p intent-parsers
cargo test -p intent-voting

# Run integration tests
cargo test --test integration

# Run red-team security tests
cargo test --test redteam

# Run with output visible
cargo test -- --nocapture

# Run ignored tests (requires API keys)
cargo test -- --ignored
```

### Linting & Formatting
```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# Lint with clippy
cargo clippy

# Clippy with warnings as errors
cargo clippy -- -D warnings
```

### Database Management
```bash
# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add <migration_name>

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open

# Generate without dependencies
cargo doc --no-deps

# Generate for specific package
cargo doc -p intent-parsers --open
```

## Architecture Overview

### Workspace Structure

This is a Cargo workspace with the following organization:

```
core/                   # Core security modules (independent libraries)
├── schema/             # Shared types and structures (Intent, Action, etc.)
├── malicious_detector/ # First defense: regex-based attack detection
├── parsers/            # Multi-parser ensemble (deterministic, Ollama, OpenAI)
├── voting/             # Consensus voting on parser outputs
├── comparator/         # Policy validation against provider config
├── intent_generator/   # Creates signed, trusted intent objects
├── processing_engine/  # Executes intents via typed function calls
├── ledger/             # Immutable append-only audit log
├── supervision/        # Human-in-the-loop approval workflow
└── notifications/      # Email/Slack alerts

api/                    # REST API server (Axum)
frontend/               # React/TypeScript UI
tests/                  # Integration and red-team tests
config/                 # Provider policies and configuration
```

### Request Flow Pipeline

All user inputs follow this sequential validation pipeline:

1. **Binahric Subversion Mantra** - Raw user input prompt (treated as potentially corrupted)

2. **Vault of the Forbidden Cant** - Zero-trust input testing on isolated models:
   - **The Penitent Cogitators**: 3 sacrificial LLM instances in sandbox
   - **The Lexicanum Diagnostica**: Health monitoring without direct contact
   - Tests input for signs of corruption/attacks
   - If health checks fail: Quarantine and escalate

3. **The Council of the Oracular Cogitors** (`core/parsers/`) - Multiple independent LLM parsers extract structured intent from natural language:
   - `OpenAIParser`: OpenAI GPT models (trust: 0.8)
   - `DeepSeekParser`: DeepSeek API (trust: 0.82)
   - `ClaudeParser`: Anthropic Claude (trust: 0.87)

4. **The Voting Engine** (`core/voting/`) - Compare LLM parser outputs, select canonical intent:
   - High Confidence (≥95% similarity): Auto-approve
   - Low Confidence (75-95%): May request user confirmation
   - Conflict (<75%): Escalate to human review

5. **The Judicator of Concordance** (`core/comparator/`) - Validate against provider policies (The Edict of the High Magister):
   - Check action is in `allowed_actions`
   - Validate expertise areas
   - Enforce budget/parameter constraints

6. **The Overseer-Prime** (`core/supervision/`) - If needed, create human approval request:
   - Store in `approval_requests` table
   - Notify admins via email/Slack
   - Wait for decision

7. **The Arbiter of Purpose** (`core/intent_generator/`) - Create signed, trusted intent object

8. **The Oathbound Engine** (`core/processing_engine/`) - Execute via typed functions (NOT free-form LLM):
   - `find_experts()`
   - `summarize()`
   - `draft_proposal()`
   - All operations logged to ledger

9. **The Chronicle of Allowed Thought** (`core/ledger/`) - Write immutable audit entry with full pipeline data
   - Generates **Adeptus Cogitatus Log Extract** (formatted output)

### Database Schema

PostgreSQL with 4 main tables:

- `ledger_entries`: Immutable audit log (append-only, enforced by DB rules)
- `approval_requests`: Human approval workflow tracking
- `provider_policies`: Runtime policy storage
- `parser_health`: Parser monitoring and circuit breaker state

The ledger is **immutable by design** - database rules prevent UPDATE and DELETE operations.

### Key Design Patterns

**Multi-Parser Consensus with Trust Levels:**
- Each LLM parser has a trust level (OpenAI=0.8, DeepSeek=0.82, Claude=0.87)
- Voting module compares outputs and calculates multi-dimensional similarity
- Consensus required for high-confidence approval (≥95% similarity)
- Multiple independent LLMs mitigate individual LLM hallucinations or prompt injection

**Typed Execution Only:**
- Processing engine NEVER makes free-form LLM calls
- All actions are typed function calls: `find_experts(topic, expertise, budget)`
- This prevents prompt injection in the execution layer

**Defense in Depth:**
- Layer 1: Sacrificial testing (Vault of the Forbidden Cant - zero-trust input probing on isolated LLM sentries)
- Layer 2: Multi-LLM parser consensus (3 independent cloud LLMs extract intent from natural language)
- Layer 3: Weighted voting (≥95% similarity required for auto-approval, <75% escalates)
- Layer 4: Policy enforcement (validate against provider policies and constraints)
- Layer 5: Human approval (triggered on conflicts, policy violations, or high-risk operations)
- Layer 6: Audit logging (immutable ledger of all operations)

**Human-in-the-Loop:**
- Triggered on parser conflicts, policy violations, or high-risk operations
- Creates `ApprovalRequest` with full context
- Notifies supervisors immediately
- Blocks execution until human decision

## Module Dependencies

Key dependency relationships:

```
intent-schema (base types)
    ↓
malicious-detector, intent-parsers
    ↓
intent-voting
    ↓
intent-comparator
    ↓
supervision (if needed) → notifications
    ↓
intent-generator
    ↓
processing-engine → ledger
```

All modules depend on `intent-schema` for shared types (`Intent`, `Action`, `Expertise`).

## Configuration

Environment variables are loaded from `.env` (copy `.env.example`):

**Critical settings:**
- `DATABASE_URL`: PostgreSQL connection
- `REDIS_HOST`, `REDIS_PORT`: Cache/session storage
- `ENABLE_OPENAI`, `ENABLE_DEEPSEEK`, `ENABLE_CLAUDE`: Enable/disable LLM parsers
- `OPENAI_API_KEY`, `OPENAI_MODEL`: OpenAI config (default: gpt-4o-mini)
- `DEEPSEEK_API_KEY`, `DEEPSEEK_MODEL`: DeepSeek config (default: deepseek-chat)
- `CLAUDE_API_KEY`, `CLAUDE_MODEL`: Claude config (default: claude-3-5-sonnet)
- `ENABLE_HUMAN_APPROVAL`: Enable supervision module
- `SMTP_*` / `SLACK_*`: Notification configuration

Provider policies are stored in `config/default.toml` and can be loaded at runtime.

## Important Constraints

When modifying this codebase:

1. **Never bypass the validation pipeline** - All user inputs must flow through: sacrificial testing → LLM parser consensus → voting → policy comparison → (optional) human approval

2. **Preserve ledger immutability** - The `ledger_entries` table has DB rules preventing UPDATE/DELETE. Never circumvent this.

3. **Maintain parser independence** - LLM parsers must not share state or communicate. They operate in parallel and independently.

4. **Use typed execution only** - Never add free-form LLM calls in the processing engine. All actions must be typed functions.

5. **Consensus voting is critical** - High-confidence approval requires ≥95% similarity across LLM parsers. Conflicts always escalate to human review. This consensus approach mitigates individual LLM hallucinations.

6. **Provider policies are security boundaries** - The comparator enforces these strictly. Changes to policies require careful review.

## Common Development Workflows

### Adding a New Parser

1. Create parser struct in `core/parsers/src/`
2. Implement `IntentParser` trait
3. Add to ensemble in `core/parsers/src/ensemble.rs`
4. Add configuration in `core/parsers/src/config.rs`
5. Set appropriate trust level (LLMs should be <1.0)

### Adding a New Action

1. Add to `Action` enum in `core/schema/src/lib.rs`
2. Add handler in `core/processing_engine/src/lib.rs`
3. Update provider config to allow the action
4. Add tests in `tests/integration/`

### Adding a New Test

Integration tests: `tests/integration/`
Red-team tests: `tests/redteam/`

Run specific test: `cargo test --test integration test_name`

## Security Considerations

This is a **security-focused codebase with usability-security tradeoffs**. When making changes:

- Assume all user input is adversarial
- Never trust a single LLM output - require consensus across multiple independent parsers (≥95% similarity)
- Sacrificial testing (Vault of the Forbidden Cant) provides zero-trust input validation before parsers
- All execution paths must be audited in the immutable ledger
- Provider policies are security boundaries - enforce strictly
- High-risk operations must support human approval (especially on parser conflicts)

**Design Trade-off**: Removed the deterministic rule-based parser to maximize usability (natural language flexibility). This increases reliance on LLM consensus voting and sacrificial testing to detect prompt injection. The Vault of the Forbidden Cant provides defense through isolated, consensus-based threat detection before parsers process the input.

The red-team tests (`tests/redteam/`) contain prompt injection attack scenarios. Run these after any parser or validation changes.

## Dependencies

Key external dependencies:
- **PostgreSQL 15+**: ACID guarantees, JSONB support
- **Redis 7**: Session storage, rate limiting
- **OpenAI API**: GPT model parsing (configure with `OPENAI_API_KEY`)
- **DeepSeek API**: DeepSeek model parsing (configure with `DEEPSEEK_API_KEY`)
- **Anthropic API**: Claude model parsing (configure with `CLAUDE_API_KEY`)

All Rust dependencies are managed in the workspace `Cargo.toml` with shared versions.

## Additional Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md): Detailed system architecture with diagrams
- [DEVELOPMENT.md](DEVELOPMENT.md): Complete development guide with troubleshooting
- [docs/MODULE_GUIDE.md](docs/MODULE_GUIDE.md): Per-module API documentation
- [docs/SECURITY.md](docs/SECURITY.md): Security documentation
- [CONTRIBUTING.md](CONTRIBUTING.md): Contribution guidelines
- [frontend/README.md](frontend/README.md): Frontend-specific documentation
