# alloy

> **Tầng 0 — Meta-crate** · Unified entry point · Tower middleware stack

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## Tổng quan

`alloy` là meta-crate tổng hợp toàn bộ Wikialisalabs EVM stack. Thêm một dependency duy nhất và chọn đúng features bạn cần.

```
alloy (meta-crate)
 ├── alloy-core    (Tầng 1 — primitives)
 ├── alloy-evm     (Tầng 2 — EVM interpreter)
 ├── alloy-trie    (Tầng 2 — Merkle Patricia Trie)
 ├── alloy-evm2    (Tầng 2 — experimental parallel EVM)
 ├── alloy-eips    (Tầng 3 — hardfork activation)
 ├── op-alloy      (Tầng 3 — OP Stack types)
 └── svm-rs        (Tầng 3 — solc version manager)
```

---

## Cài đặt

```toml
# Mặc định (core + evm + eips)
[dependencies]
alloy = { git = "https://github.com/wikialisalabs/alloy" }

# Tất cả tính năng
[dependencies]
alloy = { git = "https://github.com/wikialisalabs/alloy", features = ["full"] }

# Chỉ core + evm2 + trie
[dependencies]
alloy = { git = "https://github.com/wikialisalabs/alloy", features = ["evm2", "trie"], default-features = false }
```

## Feature Flags

| Flag | Bao gồm | Mặc định |
|------|---------|----------|
| `core` | `alloy-core` | ✅ |
| `evm` | `alloy-evm` | ✅ |
| `eips` | `alloy-eips` | ✅ |
| `evm2` | `alloy-evm2` | ❌ |
| `trie` | `alloy-trie` | ❌ |
| `op` | `op-alloy` | ❌ |
| `svm` | `svm-rs` | ❌ |
| `full` | Tất cả | ❌ |

---

## Sử dụng nhanh

```rust
use alloy::prelude::*;

// Primitives
let addr = Address::from_bytes([0xde; 20]);
let zero = U256::ZERO;

// Run EVM
let mut ctx = EvmContext::new(vec![0x00], 21_000); // STOP
let (reason, _) = run_simple(&mut ctx);
assert_eq!(reason, ExitReason::Succeed);

// Hardfork check
let active = HardforkConfig {
    hardfork: Hardfork::Prague,
    block: None,
    timestamp: Some(1_746_000_000),
}.is_active_at(0, 1_800_000_000);
println!("Prague active: {active}");
```

---

## Middleware Stack

```rust
use alloy::middleware::{ExecutionMiddleware, LoggingMiddleware, EvmRequest, Middleware};

let executor = ExecutionMiddleware;
let logger = LoggingMiddleware { label: "debug" };

let req = EvmRequest { tx: vec![0x60, 0x01, 0x00], block: 1 };
let resp = logger.call(req, &executor);
println!("Gas used: {}", resp.gas_used);
```

---

## Kiểm thử

```bash
cargo test
cargo test --features full
```

---

## Đóng góp

Xem [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Giấy phép

Apache-2.0 © 2026 Wikialisalabs Foundation
