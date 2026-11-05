# Arise and Shine bookkeeping

A phone-first bookkeeping app for the women and youth groups that make and sell Arise and Shine soap and lotion in Mbulu, Tanzania. Groups record what they produced (with the ingredients the government asks for) and what they sold. The app works out profit and warns when a group is losing money.

The design is described in [docs/architecture.md](docs/architecture.md), and project background is in [background/](background/README.md).

## Run it locally

You need Rust, the WASM target and `cargo-leptos`:

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install cargo-leptos --locked
```

Then:

```bash
cp .env.example .env
```

```bash
cargo leptos watch
```

Open http://localhost:3000. With the example `.env` you get demo data and these logins, all with PIN `1234`:

| Phone | Who |
|---|---|
| 0700000000 | ASE administrator (sees every group, manages groups, members and prices) |
| 0700000001 | Mama Neema, leader of the demo group Amka Kumekucha |
| 0700000002 | Rehema, member of Amka Kumekucha |

The database is the file `arise_shine.db`. Delete it to start over.

Run the tests with:

```bash
cargo test --features ssr
```

## Language

The interface is in English and Swahili. All text lives in one table in [src/i18n.rs](src/i18n.rs), with English and Swahili side by side. A new key needs both, or the code won't compile.

Which language a page uses:

1. the language the person picked on the **More** tab (saved in a `lang` cookie);
2. otherwise `DEFAULT_LANG` (`en` or `sw`);
3. otherwise English in debug builds and Swahili in release builds.

So developers see English locally and the groups see Swahili in production. The Swahili translations need a review by a native speaker before launch.

## How it's built

- **Leptos 0.8** with server-side rendering on **Axum**. Pages are HTML from the server, and forms are real HTML forms that post to server functions, so recording a sale works before the WASM has loaded on a slow connection. Once it has loaded, forms submit without a page reload and show live totals.
- **SQLite** through `sqlx`, in WAL mode. The schema is in [migrations/](migrations/).
- A **double-entry ledger** in [src/server/ledger.rs](src/server/ledger.rs). Every movement of money or stock is a transfer between two accounts. Transfers are append-only (database triggers refuse updates and deletes), so mistakes are corrected with "Undo", which adds reversing transfers. Every form carries an idempotency key, so a double tap or a retry never records anything twice.
- **Phone number + PIN** login with argon2 hashes, 30-day sessions and a limit of 5 failed attempts per 15 minutes.

```
src/
  app.rs          page shell, routes, top bar and bottom menu bar
  api.rs          server functions (everything the browser can ask for)
  pages/          one file per tab: home, production, sales, more, admin, login
  components.rs   icons, notes, small helpers
  i18n.rs         English and Swahili text
  model.rs        types shared by server and browser; quantity and money formatting
  error.rs        error codes, translated in the UI
  server/         server only: ledger, read queries, auth, database, seed data
style/main.css    styles, using the Arise and Shine design tokens
public/patterns/  sunflower kitenge and kanga patterns from the design system
```

Leptos rule worth knowing: create resources (`Resource::new`) at the top of a component, never inside a `Suspend` block. The server can run a `Suspend` block more than once, and the browser then matches resources to the wrong data and fails to hydrate.

## Prices

Liquid soap at TSh 2,000 per litre is the real district price. **Every other price in the seed data is a placeholder**: bar soap, lotion and all raw materials. The ASE administrator can change prices on the Administration page (More → Administration). Old prices are kept, so past records keep their history.

## Deploy

The deployable unit is one Docker image plus one database file:

```bash
cp .env.example .env.production
```

Edit `.env.production`. Set a real `ADMIN_PHONE` and `ADMIN_PIN`, remove `SEED_DEMO` and `DEFAULT_LANG`, set `COOKIE_SECURE=1` behind HTTPS, and fill in the Litestream settings. Then:

```bash
docker compose up -d --build
```

With `LITESTREAM_REPLICA_URL` set, the container streams every database change to S3-compatible storage (Backblaze B2, Cloudflare R2). On a new host with an empty volume, it restores the latest copy first. Moving hosts means starting the container on the new host with the same Litestream settings and pointing the domain at it.

Put a TLS-terminating proxy (for example Caddy) in front of port 3000.

## Deploy

Every push to `main` builds the Docker image in GitHub Actions ([.github/workflows/docker.yml](.github/workflows/docker.yml)) and publishes it as `ghcr.io/arise-and-shine-excellence/arise-shine`. The server runs [deploy/compose.yml](deploy/compose.yml), where Watchtower checks for a new image every 5 minutes and restarts the app with it. The database stays in the `data` volume.

The server is LXC container 105 (`arise-shine`) on the Proxmox host, with the stack in `/opt/arise-shine` and its settings in `/opt/arise-shine/.env.production`. Nginx Proxy Manager forwards the domain to port 3000.
