# Architecture

Status: proposal, November 2026. Written after the [2026-11-04 software planning meeting](../background/meetings/2026-11-04-arise-and-shine-software-planning.md).

## What we're building

A bookkeeping app for the women and youth groups that make and sell Arise and Shine products. The MVP covers two things:

1. **Production records:** what a group produced, how much, the cost, and the ingredients used. The government requires ingredient records, for example how much beeswax went into a batch.
2. **Sales:** what was sold, how many and for how much. Profit is calculated automatically, and the app alerts the group when it's losing money.

Planned for later, in this order:

- Group and member tracking (what each member sold this month).
- A management dashboard for Lujo and EWB.
- NGO annual and quarterly reports, and a donor panel.
- Possibly other small businesses (shopkeepers) using the same stock-in/stock-out model.

## Constraints

| Constraint | Source | Consequence |
|---|---|---|
| Users are on smartphones with slow, unreliable internet | 2026-10-19 meeting | Server-rendered pages, small payloads, forms that work without JavaScript |
| Fewer than 100 groups, roughly 30 today | 2026-11-04 meeting | One server and one embedded database is enough |
| Hosting must be cheap and eventually in Tanzania | 2026-10-19 and 2026-11-04 meetings | One container image and one database file, movable between hosts |
| MVP runs on Andreas's home server before funding | 2026-11-04 meeting | Must be easy to move later |
| The interface is mostly Swahili | Existing BridgeGap prototype | i18n from day one |
| Prices are fixed across the district | 2026-11-04 meeting | Prices are configuration, not something each sale negotiates |
| Lujo wants to own and understand the code | 2026-10-19 meeting | See [Open questions](#open-questions) |

## Stack

```
phone browser
   │  HTML over HTTPS (works before WASM loads)
   ▼
Leptos SSR + Axum  ── single Rust binary in a Docker image
   │  sqlx
   ▼
SQLite file  ──  Litestream  ──▶  off-site object storage (backups)
```

- **Leptos with server-side rendering, on Axum.** Every page is rendered on the server. Forms use `ActionForm` and server functions, so they submit as plain HTML forms before the WASM bundle has loaded and are enhanced once it has. Use islands mode if the hydration bundle grows too large for slow connections.
- **SQLite through `sqlx`.** One file, no separate database server. Run it in WAL mode. `sqlx` keeps a move to Postgres possible if we ever need several app servers.
- **Litestream** streams the database to cheap S3-compatible storage (for example Backblaze B2 or Cloudflare R2) for continuous off-site backups. This matters more than usual because a home server and some local hosts are exposed to power and internet outages.
- **Docker.** The deployable unit is one image plus one database file. Moving from the home server to a Tanzanian VPS means copying both and pointing the domain at the new host.
- **`leptos_i18n`** for Swahili and English.

### Why not TigerBeetle

TigerBeetle is a purpose-built financial ledger database. It's a good project but the wrong fit here:

- **Scale.** We expect a few thousand entries a month. SQLite handles thousands of writes per second, while TigerBeetle is built for millions of transfers per second.
- **It only stores numbers.** Accounts and transfers are fixed-size integer records with no text fields. Product names, ingredient notes, groups and members would still need a second database, so we'd run two systems instead of one.
- **Operations.** A fault-tolerant TigerBeetle deployment is a cluster of replicas. That works against cheap hosting maintained by a small team.

We keep TigerBeetle's ideas and build them in SQL tables. See [Ledger model](#ledger-model).

## Ledger model

All money and stock goes through one double-entry ledger. Kristoffer owns the bookkeeping design, and this section is a starting point for him, not a final schema.

### Principles

1. **Double entry.** Every movement is a transfer from one account to another, so value is never created or lost silently.
2. **Append-only.** Transfers are never updated or deleted. A mistake is corrected with a new reversing transfer, which gives a full audit trail for donors and the government.
3. **Idempotent writes.** Every form submission carries a client-generated UUID, and the database rejects a second transfer with the same key. A retry over a flaky connection, or revisiting a page (the bug in the current payment page), can never record anything twice.
4. **Balances are derived** from transfers. Cache them only if it's ever measured to be necessary.
5. **One ledger per asset.** Shillings, litres of liquid soap, soap bars and kilograms of caustic soda are separate ledgers. A transfer always moves one asset, and a sale is two linked transfers (stock out, money in).
6. **Integer amounts.** Store shillings as whole TZS and stock in the smallest unit we count (bars, millilitres, grams). Never use floats.

### Sketch

```sql
organisation (id, name, kind)              -- ASE, each group, later shopkeepers
member       (id, organisation_id, name, phone, pin_hash, role)
asset        (id, code, unit)              -- 'TZS', 'liquid_soap_ml', 'soap_bar', 'naoh_g'
account      (id, organisation_id, asset_id, kind)
                                          -- kind: cash, stock, raw_materials, sales, cost, external
transfer     (id, idempotency_key UNIQUE, debit_account_id, credit_account_id,
              amount, created_at, created_by, batch_id NULL, sale_id NULL, reverses_id NULL)
batch        (id, organisation_id, site, product, produced_at, comment)
batch_ingredient (batch_id, asset_id, amount)   -- the government ingredient record
sale         (id, organisation_id, member_id, sold_at, comment)
price        (asset_id, kind, amount_tzs, valid_from)  -- fixed district prices
```

Every row is scoped to an `organisation`, so adding shopkeepers later needs no redesign.

### Example flows

- **A group buys raw materials from ASE:** the group's cash account pays ASE's cash account (TZS), and ASE's raw-materials account moves stock to the group's raw-materials account.
- **A production batch:** raw materials move from the group's raw-materials account to a production cost account, and finished product moves from an external "produced" account into the group's stock. The `batch` and `batch_ingredient` rows carry the details the government needs.
- **A sale:** stock moves from the group's stock account to an external customer account, and TZS moves from the customer account to the group's cash account, at the fixed price.
- **Profit** for a period is sales revenue minus the cost of materials, read straight from the ledger.

## Authentication

- Lujo creates groups and members.
- Members log in with their phone number and a PIN. Store hashed PINs (argon2) and rate-limit login attempts.
- No SMS one-time codes in the MVP, because each message costs money.
- Roles: member, group leader, ASE admin, EWB viewer (read-only dashboard).

## Hosting plan

| Phase | Where | Notes |
|---|---|---|
| MVP | Andreas's home server, own domain | Before funding. Use Litestream backups from day one. |
| Funded | VPS with a Tanzanian provider | Put hosting and domain costs in the funding plan. Check that the VPS is actually located in Tanzania, since one plan seen in the 2026-10-19 meeting was in the USA. |

Resource needs are small: the smallest VPS tier (around 1 vCPU and 1 GB RAM) is plenty.

## Not in the MVP

- **Offline sync.** If Lujo's user interviews show people are often completely offline, add a PWA that queues entries on the phone and sends them later. Idempotency keys make this safe to add.
- A native app. A responsive website avoids app store reviews (agreed 2026-10-13).
- Donor panel, NGO reports, QR scanning, M-PESA or VICOBA integration.

## Open questions

1. **Who maintains the code long term?** Lujo wants to own the code and was leaning toward plain HTML/CSS. Rust and Leptos have a steep learning curve, even with AI coding tools, although the compiler catches many AI mistakes. Agree on this with Lujo before committing to the stack.
2. **Data protection.** Tanzania's Personal Data Protection Act (2022) appears to restrict sending personal data abroad and may require registering as a data controller. Check whether hosting member data in Denmark during the MVP is allowed, and what registration ASE needs.
3. **The exact bookkeeping rules** (Kristoffer): which accounts each flow touches, how ASE's chemical margin is recorded, and what the annual NGO report needs.
4. **Members versus groups.** Do individual members record sales, or only the group leader?
