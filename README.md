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
git clone https://github.com/your-org/solana-anchor-token2022-starter.git
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

## 📄 License & Commercial Distribution

[![Get Pro Edition on Polar](https://img.shields.io/badge/Polar.sh-Instant_Checkout_%2449-blue?style=for-the-badge&logo=github)](https://buy.polar.sh/polar_cl_4yZQH8N57BZNYhFowHUSqE4vvUAMzYXeA5r0f2zbD50)

- **Community Edition:** Free & Open Source under the MIT License.
- **Pro / Commercial Edition ($49):** Includes priority Discord support, advanced Token-2022 confidential transfer extensions, and production deployment scripts. 👉 [**Instant Checkout on Polar.sh ($49)**](https://buy.polar.sh/polar_cl_4yZQH8N57BZNYhFowHUSqE4vvUAMzYXeA5r0f2zbD50)
