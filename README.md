# ⚡ Solana Anchor 0.30+ & Token-2022 Production Starter Kit

A production-grade, audited-pattern boilerplate engineered for building scalable Solana dApps and protocols using the latest **Anchor 0.30+** framework and **Token-2022 (SPL Token Extensions)**.

---

## 🌟 What's Included

- 🔒 **Token-2022 Transfer Hook Program:** Enforce on-chain logic, custom protocol royalties, or compliance whitelisting on every transfer.
- 📦 **Monorepo Architecture (pnpm Workspaces):**
  - `programs/token-suite`: Anchor 0.30+ smart contract with modular instructions.
  - `packages/sdk`: Clean TypeScript client SDK with automatic priority fee routing.
  - `packages/ui-hooks`: React hooks ready for `@solana/wallet-adapter`.
- ⚡ **Automated CI/CD:** GitHub Actions workflow testing rustfmt, clippy, and TypeScript typechecking.
- 🧪 **Comprehensive Test Suite:** Mocha/Chai integration and deterministic PDA derivation checks.

---

## 🚀 Quick Start

### 1. Prerequisites
Ensure you have the following installed:
- [Rust](https://rustup.rs/) (1.75.0+)
- [Node.js](https://nodejs.org/) (v20+) & [pnpm](https://pnpm.io/)
- [Solana CLI](https://docs.solanalabs.com/cli/install) (1.18.26+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (0.30.1)

### 2. Installation
```bash
# Clone the repository
git clone https://github.com/sidsri14/solana-anchor-token2022-starter.git
cd solana-anchor-token2022-starter

# Install monorepo dependencies
pnpm install
```

### 3. Build & Test
```bash
# Build the Anchor program
anchor build

# Run TypeScript tests
pnpm test
```

---

## 🏗️ Architecture

```
solana-anchor-token2022-starter/
├── programs/
│   └── token-suite/
│       ├── src/
│       │   ├── instructions/
│       │   │   ├── initialize_mint.rs   # Sets up Token-2022 Mint & Protocol PDA
│       │   │   ├── mint_tokens.rs       # Secure CPI minting logic
│       │   │   └── transfer_hook.rs     # SPL Transfer Hook interface implementation
│       │   ├── state/
│       │   │   └── config.rs            # Protocol state space & seeds
│       │   ├── errors.rs                # Typed programmatic error codes
│       │   └── lib.rs                   # Entrypoint & program declaration
├── packages/
│   ├── sdk/                             # Standalone client SDK
│   └── ui-hooks/                        # React & Wallet-Adapter hooks
├── tests/                               # Integration tests
└── .github/workflows/                   # Automated GitHub Actions CI
```

---

## 📄 License & Custom Smart Contract Engineering

- **Open Source License:** MIT License (`LICENSE`).
- **Custom Anchor / Token-2022 Engineering ($60/hr):** Need custom transfer hook programs, fee mechanics, PDA architecture, or Rust code reviews? Reach out via **`sidsri1502@gmail.com`**, inspect [`sidsri14.github.io`](https://sidsri14.github.io/), or [Hire on Upwork](https://www.upwork.com/freelancers/~01f93f1a56b9ef1b8d).
