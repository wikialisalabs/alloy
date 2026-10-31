# Hướng dẫn Đóng góp — `alloy` (meta-crate)

**`alloy`** là điểm vào thống nhất của toàn bộ stack — đóng góp ở đây chủ yếu là: thêm re-export, thêm feature flag, cải thiện middleware, hoặc cập nhật prelude.

---

## Mục lục

1. [Môi trường Phát triển](#1-môi-trường-phát-triển)
2. [Cấu trúc Repository](#2-cấu-trúc-repository)
3. [Quy trình Đóng góp](#3-quy-trình-đóng-góp)
4. [Thêm Sub-crate mới](#4-thêm-sub-crate-mới)
5. [Thêm Middleware mới](#5-thêm-middleware-mới)
6. [Cập nhật Prelude](#6-cập nhật-prelude)
7. [Viết Tests](#7-viết-tests)
8. [Commit Convention](#8-commit-convention)
9. [Pull Request Checklist](#9-pull-request-checklist)

---

## 1. Môi trường Phát triển

```bash
git clone https://github.com/wikialisalabs/alloy.git
cd alloy

# Cần các sub-crate trong monorepo
# (hoặc đặt path dependencies đúng trong Cargo.toml)

cargo build --features full
cargo test --features full
```

---

## 2. Cấu trúc Repository

```
alloy/
├── src/
│   ├── lib.rs          # Meta-crate root, re-exports, feature gates
│   ├── middleware.rs   # Tower-compatible middleware trait + built-in layers
│   └── prelude.rs      # Glob-import convenience re-exports
├── Cargo.toml
├── README.md
└── CONTRIBUTING.md
```

---

## 3. Quy trình Đóng góp

1. Kiểm tra [Issues](https://github.com/wikialisalabs/alloy/issues).
2. Fork → branch.
3. Implement + tests.
4. CI pass với `--features full`.
5. Mở PR.

---

## 4. Thêm Sub-crate mới

Khi thêm một crate mới vào stack (e.g. `hardforks`):

1. Thêm optional dependency vào `Cargo.toml`:
   ```toml
   hardforks = { path = "../hardforks", optional = true }
   ```
2. Thêm feature flag:
   ```toml
   [features]
   hardforks = ["dep:hardforks"]
   full = [..., "hardforks"]
   ```
3. Thêm re-export trong `src/lib.rs`:
   ```rust
   #[cfg(feature = "hardforks")]
   pub use hardforks;
   ```
4. Thêm prelude re-exports trong `src/prelude.rs` nếu có types thường dùng.
5. Cập nhật bảng Feature Flags trong `README.md`.

---

## 5. Thêm Middleware mới

Thêm struct implement `Middleware` trait trong `src/middleware.rs`:

```rust
/// Rate-limiting middleware.
pub struct RateLimitMiddleware {
    pub max_calls_per_block: u64,
    counter: std::sync::atomic::AtomicU64,
}

impl Middleware for RateLimitMiddleware {
    fn call(&self, req: EvmRequest, next: &dyn Middleware) -> EvmResponse {
        // ... rate limiting logic
        next.call(req, next)
    }
}
```

Viết tests cho cả case bình thường và case bị giới hạn.

---

## 6. Cập nhật Prelude

Thêm type phổ biến vào `src/prelude.rs` sau khi thảo luận trong Issue.  
Nguyên tắc: chỉ thêm types mà **>80% người dùng** sẽ dùng.

---

## 7. Viết Tests

```bash
# Test với tất cả features
cargo test --features full

# Test default features
cargo test
```

---

## 8. Commit Convention

```
feat(alloy): add hardforks sub-crate re-export under feature flag
feat(middleware): add retry middleware with exponential backoff
fix(prelude): remove ambiguous re-export causing name conflict
chore(alloy): bump sub-crate dependency versions
```

---

## 9. Pull Request Checklist

```
[ ] cargo fmt --all                          → no diff
[ ] cargo clippy --features full -- -D warnings → 0 warnings
[ ] cargo test --features full               → all pass
[ ] cargo test (default features)            → all pass
[ ] Feature flags documented in README       ✓
[ ] Prelude change discussed in Issue        ✓ (if prelude modified)
[ ] PR linked to Issue                       ✓
```

---

## Liên hệ

- **Email:** root@wikialisalabs.axvn.vn
- **Issues:** https://github.com/wikialisalabs/alloy/issues

*Cảm ơn bạn đã đóng góp! 🦀🏗️*
