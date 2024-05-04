# LM-SERVER


[![Rust Check](https://github.com/nhatnm0612/lm-server-rust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/nhatnm0612/lm-server-rust/actions/workflows/rust.yml?query=branch%3Amain)
[![Python Check](https://github.com/nhatnm0612/lm-server-rust/actions/workflows/python-app.yml/badge.svg?branch=main)](https://github.com/nhatnm0612/lm-server-rust/actions/workflows/python-app.yml?query=branch%3Amain)
[![codecov](https://codecov.io/gh/nhatnm0612/lm-server-rust/graph/badge.svg?token=5ARNRQFB42)](https://codecov.io/gh/nhatnm0612/lm-server-rust)

- [Description](#description)
- [How to run?](#how-to-run)
    - [Fake Server](#fake-server)


## Description
Running this code with `cargo run` and let it scan ports from localhost where llm server is hosted, if it found a port to use, it will write down the ip address, port and response time to an output folder.

## How to run?
Pull with git:
```bash
git pull git@github.com:nhatnm0612/lm-server-rust.git
```

Cd into new directory:
```bash
cd lm-server-rust
```

Run with cargo:
```bash
cargo build -vv
cargo run
```

Run test with cargo:
```bash
cargo test -vv
```

Run with cargo in dev environment:
```bash
ENVIRON=dev cargo run
```

### Fake Server
Fake server in `./dev/main.py` is a Python FastAPI Server mimicking OpenAI completions route. Used for development only.
