# Monad Load Balancer

An adaptive load balancer written in Rust.

## Structure

```
monad-load-balancer/
├── src/                    # Source code
│   ├── balancer/          # Load balancing logic
│   ├── health/            # Health checking system
│   ├── config/            # Configuration management
│   ├── server/            # Server/proxy implementation
│   ├── algorithms/        # Load balancing algorithms
│   └── .gitkeep
├── tests/                  # Test suite
│   ├── unit/              # Unit tests
│   ├── integration/       # Integration tests
│   └── .gitkeep
├── examples/               # Usage examples
├── benchmarks/             # Performance benchmarks
├── docs/                   # Documentation
├── scripts/                # Build/deployment scripts
└── config.yaml            # Configuration file
```