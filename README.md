# personal-website

My personal website. Live at [monolith.vision](https://monolith.vision).

## Setup + Run

- Install rust toolchain
- Install node + npm
- Have GNU `make`
- Create a `cert.pem` and a `key.pem` (ssl cert stuff)
- Get access to a mongodb server
- Fill in `.env.example` with mongodb credentials and admin `key` and move it to `.env`
- Finally, run:

```bash
make install
make build_frontend
make build_backend
make run
```

P.S: I know I should make a `dockerfile`, but I'll do that later.
