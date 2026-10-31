//! Tower-compatible middleware layer for EVM request/response pipelines.
//!
//! The middleware stack allows composable transformations on EVM calls,
//! following the same pattern as the [`tower`](https://docs.rs/tower) crate.

/// A request to execute on the EVM.
#[derive(Debug, Clone)]
pub struct EvmRequest {
    /// RLP-encoded transaction.
    pub tx: Vec<u8>,
    /// Block number context.
    pub block: u64,
}

/// The response from an EVM execution.
#[derive(Debug, Clone)]
pub struct EvmResponse {
    /// Whether the execution succeeded.
    pub success: bool,
    /// Return data.
    pub output: Vec<u8>,
    /// Gas used.
    pub gas_used: u64,
}

/// A middleware layer that can transform `EvmRequest` / `EvmResponse`.
pub trait Middleware: Send + Sync {
    /// Process the request, optionally calling the next layer.
    fn call(&self, req: EvmRequest, next: &dyn Middleware) -> EvmResponse;
}

/// The terminal middleware — executes the request (stub).
pub struct ExecutionMiddleware;

impl Middleware for ExecutionMiddleware {
    fn call(&self, req: EvmRequest, _next: &dyn Middleware) -> EvmResponse {
        // Stub: treat non-empty tx data as success
        EvmResponse {
            success: !req.tx.is_empty(),
            output: req.tx.clone(),
            gas_used: req.tx.len() as u64 * 21,
        }
    }
}

/// A logging middleware — logs requests before passing to the next layer.
pub struct LoggingMiddleware {
    /// Label printed with each log line.
    pub label: &'static str,
}

impl Middleware for LoggingMiddleware {
    fn call(&self, req: EvmRequest, next: &dyn Middleware) -> EvmResponse {
        // In production, use tracing::info! here
        let _ = &self.label; // suppress unused warning in stub
        next.call(req, next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_middleware_success() {
        let m = ExecutionMiddleware;
        let req = EvmRequest { tx: vec![0x60, 0x01], block: 1 };
        let resp = m.call(req, &m);
        assert!(resp.success);
    }

    #[test]
    fn execution_middleware_empty_tx_fails() {
        let m = ExecutionMiddleware;
        let req = EvmRequest { tx: vec![], block: 1 };
        let resp = m.call(req, &m);
        assert!(!resp.success);
    }
}
