-- Arise and Shine bookkeeping: first schema.
-- See docs/architecture.md, "Ledger model".
--
-- Conventions
--   * All money is whole Tanzanian shillings (TZS). All stock is integers in the
--     smallest unit we count (ml, g, bars). Never floats.
--   * A transfer moves `amount` of one asset from the CREDIT account (gives)
--     to the DEBIT account (receives). An account's balance is
--     SUM(debits) - SUM(credits).
--   * Transfers are append-only. Mistakes are corrected with a reversing
--     transfer that points at the original through `reverses_id`.

CREATE TABLE organisation (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    kind        TEXT NOT NULL CHECK (kind IN ('ase', 'group', 'external')),
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE member (
    id               INTEGER PRIMARY KEY,
    organisation_id  INTEGER NOT NULL REFERENCES organisation(id),
    name             TEXT NOT NULL,
    phone            TEXT NOT NULL UNIQUE,   -- normalised: digits only, local form 07xxxxxxxx
    pin_hash         TEXT NOT NULL,          -- argon2
    role             TEXT NOT NULL CHECK (role IN ('member', 'leader', 'admin', 'viewer')),
    created_at       TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE session (
    token_hash  TEXT PRIMARY KEY,            -- sha256 of the cookie value
    member_id   INTEGER NOT NULL REFERENCES member(id),
    expires_at  TEXT NOT NULL
);

CREATE TABLE login_attempt (
    id       INTEGER PRIMARY KEY,
    phone    TEXT NOT NULL,
    at       TEXT NOT NULL,
    success  INTEGER NOT NULL
);
CREATE INDEX login_attempt_phone_at ON login_attempt (phone, at);

-- What we count. `factor` is how many base units make one display unit
-- (1000 ml = 1 L), so people type "1.5" litres and we store 1500 ml.
CREATE TABLE asset (
    id        INTEGER PRIMARY KEY,
    code      TEXT NOT NULL UNIQUE,
    kind      TEXT NOT NULL CHECK (kind IN ('money', 'product', 'material')),
    unit      TEXT NOT NULL,
    factor    INTEGER NOT NULL DEFAULT 1 CHECK (factor > 0),
    unit_en   TEXT NOT NULL,
    unit_sw   TEXT NOT NULL,
    name_en   TEXT NOT NULL,
    name_sw   TEXT NOT NULL,
    sort      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE account (
    id               INTEGER PRIMARY KEY,
    organisation_id  INTEGER NOT NULL REFERENCES organisation(id),
    asset_id         INTEGER NOT NULL REFERENCES asset(id),
    kind             TEXT NOT NULL CHECK (kind IN ('cash', 'stock', 'raw_materials', 'cost', 'external')),
    UNIQUE (organisation_id, asset_id, kind)
);

CREATE TABLE batch (
    id                INTEGER PRIMARY KEY,
    idempotency_key   TEXT NOT NULL UNIQUE,
    organisation_id   INTEGER NOT NULL REFERENCES organisation(id),
    product_asset_id  INTEGER NOT NULL REFERENCES asset(id),
    quantity          INTEGER NOT NULL CHECK (quantity > 0),
    site              TEXT NOT NULL DEFAULT '',
    produced_on       TEXT NOT NULL,          -- YYYY-MM-DD
    comment           TEXT NOT NULL DEFAULT '',
    created_at        TEXT NOT NULL,
    created_by        INTEGER NOT NULL REFERENCES member(id)
);

-- The government ingredient record.
CREATE TABLE batch_ingredient (
    batch_id  INTEGER NOT NULL REFERENCES batch(id),
    asset_id  INTEGER NOT NULL REFERENCES asset(id),
    amount    INTEGER NOT NULL CHECK (amount > 0),
    PRIMARY KEY (batch_id, asset_id)
);

CREATE TABLE sale (
    id                INTEGER PRIMARY KEY,
    idempotency_key   TEXT NOT NULL UNIQUE,
    organisation_id   INTEGER NOT NULL REFERENCES organisation(id),
    member_id         INTEGER NOT NULL REFERENCES member(id),
    product_asset_id  INTEGER NOT NULL REFERENCES asset(id),
    quantity          INTEGER NOT NULL CHECK (quantity > 0),
    sold_on           TEXT NOT NULL,          -- YYYY-MM-DD
    comment           TEXT NOT NULL DEFAULT '',
    created_at        TEXT NOT NULL
);

CREATE TABLE transfer (
    id                 INTEGER PRIMARY KEY,
    idempotency_key    TEXT NOT NULL UNIQUE,
    debit_account_id   INTEGER NOT NULL REFERENCES account(id),  -- receives
    credit_account_id  INTEGER NOT NULL REFERENCES account(id),  -- gives
    amount             INTEGER NOT NULL CHECK (amount > 0),
    created_at         TEXT NOT NULL,
    created_by         INTEGER REFERENCES member(id),
    batch_id           INTEGER REFERENCES batch(id),
    sale_id            INTEGER REFERENCES sale(id),
    reverses_id        INTEGER UNIQUE REFERENCES transfer(id),
    CHECK (debit_account_id <> credit_account_id)
);
CREATE INDEX transfer_debit ON transfer (debit_account_id);
CREATE INDEX transfer_credit ON transfer (credit_account_id);
CREATE INDEX transfer_batch ON transfer (batch_id);
CREATE INDEX transfer_sale ON transfer (sale_id);

CREATE TRIGGER transfer_same_asset BEFORE INSERT ON transfer
WHEN (SELECT asset_id FROM account WHERE id = NEW.debit_account_id)
  <> (SELECT asset_id FROM account WHERE id = NEW.credit_account_id)
BEGIN
    SELECT RAISE(ABORT, 'a transfer must move a single asset');
END;

CREATE TRIGGER transfer_no_update BEFORE UPDATE ON transfer
BEGIN
    SELECT RAISE(ABORT, 'transfers are append-only');
END;

CREATE TRIGGER transfer_no_delete BEFORE DELETE ON transfer
BEGIN
    SELECT RAISE(ABORT, 'transfers are append-only');
END;

-- Fixed district prices. `amount_tzs` buys `per_quantity` base units.
-- kind 'sale': what a group sells a product for. kind 'material': what a
-- group pays ASE for a raw material. Append-only: the latest row wins.
CREATE TABLE price (
    id            INTEGER PRIMARY KEY,
    asset_id      INTEGER NOT NULL REFERENCES asset(id),
    kind          TEXT NOT NULL CHECK (kind IN ('sale', 'material')),
    amount_tzs    INTEGER NOT NULL CHECK (amount_tzs >= 0),
    per_quantity  INTEGER NOT NULL CHECK (per_quantity > 0),
    valid_from    TEXT NOT NULL
);

-- Read models derived from the ledger.
CREATE VIEW batch_view AS
SELECT b.*,
    EXISTS (SELECT 1 FROM transfer t WHERE t.batch_id = b.id AND t.reverses_id IS NOT NULL) AS reversed,
    COALESCE((SELECT SUM(t.amount) FROM transfer t
              JOIN account a ON a.id = t.credit_account_id
              WHERE t.batch_id = b.id AND t.reverses_id IS NULL
                AND a.kind = 'cash' AND a.organisation_id = b.organisation_id), 0) AS material_cost
FROM batch b;

CREATE VIEW sale_view AS
SELECT s.*,
    EXISTS (SELECT 1 FROM transfer t WHERE t.sale_id = s.id AND t.reverses_id IS NOT NULL) AS reversed,
    COALESCE((SELECT SUM(t.amount) FROM transfer t
              JOIN account a ON a.id = t.debit_account_id
              WHERE t.sale_id = s.id AND t.reverses_id IS NULL
                AND a.kind = 'cash' AND a.organisation_id = s.organisation_id), 0) AS amount_tzs
FROM sale s;

-- Fixed organisations. The app code refers to these ids.
INSERT INTO organisation (id, name, kind) VALUES
    (1, 'Arise and Shine Excellence', 'ase'),
    (2, 'Customers', 'external'),
    (3, 'Production', 'external'),
    (4, 'Suppliers', 'external');

INSERT INTO asset (id, code, kind, unit, factor, unit_en, unit_sw, name_en, name_sw, sort) VALUES
    (1,  'tzs',             'money',    'TZS', 1,    'TSh',     'TSh',     'Tanzanian shillings', 'Shilingi',          0),
    (2,  'liquid_soap_ml',  'product',  'ml',  1000, 'L',       'L',       'Liquid soap',         'Sabuni ya maji',    10),
    (3,  'soap_bar',        'product',  'bar', 1,    'bars',    'vipande', 'Soap bar',            'Sabuni ya kipande', 11),
    (4,  'lotion_bottle',   'product',  'bottle', 1, 'bottles', 'chupa',   'Organic lotion',      'Losheni asilia',    12),
    (10, 'sulphonic_acid_ml','material','ml',  1000, 'L',       'L',       'Sulphonic acid',      'Asidi ya salfoniki', 20),
    (11, 'sles_g',          'material', 'g',   1000, 'kg',      'kg',      'SLES',                'SLES',              21),
    (12, 'caustic_soda_g',  'material', 'g',   1000, 'kg',      'kg',      'Caustic soda',        'Magadi (caustic soda)', 22),
    (13, 'salt_g',          'material', 'g',   1000, 'kg',      'kg',      'Salt',                'Chumvi',            23),
    (14, 'glycerin_ml',     'material', 'ml',  1000, 'L',       'L',       'Glycerin',            'Glyserini',         24),
    (15, 'fragrance_ml',    'material', 'ml',  1000, 'L',       'L',       'Fragrance',           'Manukato',          25),
    (16, 'colour_g',        'material', 'g',   1,    'g',       'g',       'Colour',              'Rangi',             26),
    (17, 'beeswax_g',       'material', 'g',   1000, 'kg',      'kg',      'Beeswax',             'Nta ya nyuki',      27),
    (18, 'shea_butter_g',   'material', 'g',   1000, 'kg',      'kg',      'Shea butter',         'Siagi ya shea',     28),
    (19, 'sunflower_oil_ml','material', 'ml',  1000, 'L',       'L',       'Sunflower oil',       'Mafuta ya alizeti', 29);

-- Liquid soap at TSh 2,000 per litre is the real district price (2026-11-04
-- meeting). Every other price is a PLACEHOLDER until ASE confirms it; ASE
-- admins can change prices in the app.
INSERT INTO price (asset_id, kind, amount_tzs, per_quantity, valid_from) VALUES
    (2,  'sale',     2000,  1000, '2026-01-01T00:00:00Z'),
    (3,  'sale',     1000,  1,    '2026-01-01T00:00:00Z'),
    (4,  'sale',     5000,  1,    '2026-01-01T00:00:00Z'),
    (10, 'material', 8000,  1000, '2026-01-01T00:00:00Z'),
    (11, 'material', 7000,  1000, '2026-01-01T00:00:00Z'),
    (12, 'material', 4000,  1000, '2026-01-01T00:00:00Z'),
    (13, 'material', 1000,  1000, '2026-01-01T00:00:00Z'),
    (14, 'material', 10000, 1000, '2026-01-01T00:00:00Z'),
    (15, 'material', 40000, 1000, '2026-01-01T00:00:00Z'),
    (16, 'material', 50,    1,    '2026-01-01T00:00:00Z'),
    (17, 'material', 20000, 1000, '2026-01-01T00:00:00Z'),
    (18, 'material', 15000, 1000, '2026-01-01T00:00:00Z'),
    (19, 'material', 6000,  1000, '2026-01-01T00:00:00Z');
