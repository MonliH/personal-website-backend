# personal-website-backend

The backend for my personal website. Live at [monolith.vision](https://monolith.vision).

## Setup + Run

- Install rust toolchain
- Have GNU `make`
- Get access to a mongodb server
- Fill in `.env.example` with mongodb credentials and admin `key` and move it to `.env`
- Finally, run:

```bash
make build
make run
```

P.S: I know I should make a `dockerfile`, but I'll do that later.
