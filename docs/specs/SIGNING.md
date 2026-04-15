# Release Signing Infrastructure

## Overview

All Freedom App release artifacts are signed with [minisign](https://jedisct1.github.io/minisign/)
using ed25519 keys. Signatures are uploaded alongside binaries in every GitHub release.

Two keys exist:

| Key | Purpose | Baked into LibreWin OS? |
|-----|---------|------------------------|
| `librewin-releases` | Production — tag pushes only | Yes |
| `librewin-releases-dev` | Dev — non-tag CI builds | No |

## Public keys

```
# Production (librewin-releases)
# Key ID: 0531C1E41A7F05AF
RWSvBX8a5MExBbOsCmBqDLxNBb8ofBef1k3eqI79Z/LSGp/DBj1YwW5S

# Dev (librewin-releases-dev)
# Key ID: 676C01F6F2C480D5
RWTVgMTy9gFsZ8vg/cS+uQ2/hmS2LyiWMu8wXpTU7al0FJfYZPSm6n7E
```

Public key files are committed at `keys/librewin-releases.pub` and
`keys/librewin-releases-dev.pub`.

## What gets signed

For every release, each of the following is signed:

- `{app}-x86_64` — x86_64 binary
- `{app}-aarch64` — aarch64 binary
- `{app}.desktop` — OS integration contract

Signature files use the `.minisig` extension, e.g. `shelf-x86_64.minisig`.

## Verifying a release

Install minisign (available in most package managers: `apt install minisign`,
`brew install minisign`, `pacman -S minisign`).

```bash
# Download the binary, its signature, and the public key
curl -LO https://github.com/eldo9000/Libre-Apps/releases/latest/download/shelf-x86_64
curl -LO https://github.com/eldo9000/Libre-Apps/releases/latest/download/shelf-x86_64.minisig

# Verify using the inline public key (no key file needed)
minisign -Vm shelf-x86_64 -P RWSvBX8a5MExBbOsCmBqDLxNBb8ofBef1k3eqI79Z/LSGp/DBj1YwW5S

# Or verify using the key file
curl -LO https://raw.githubusercontent.com/eldo9000/Libre-Apps/main/keys/librewin-releases.pub
minisign -Vm shelf-x86_64 -p librewin-releases.pub
```

Expected output on success:
```
Signature and comment signature verified
Trusted comment: librewin-releases v0.2.0 shelf-x86_64
```

## How LibreWin OS uses this

The LibreWin OS install manager (`LibreWin-OS/build/fetch-apps.sh`) embeds the production
public key inline. Before installing any binary it runs:

```bash
minisign -Vm "$binary" -P "$LIBREWIN_RELEASES_PUBKEY"
```

If verification fails, the install aborts. The key is never fetched from the network at
install time — it is baked into the build at ISO creation.

## Private key storage

Private keys are stored exclusively as GitHub Actions secrets:

- `MINISIGN_KEY` — production key (used on tag push)
- `MINISIGN_KEY_DEV` — dev key (used on non-tag builds)

No developer holds a copy. See `keys/ROTATION.md` for the rotation procedure.

## Signing in CI

The workflow writes the secret to a temp file, signs, then deletes immediately:

```yaml
- name: Sign artifacts
  env:
    MINISIGN_KEY: ${{ secrets.MINISIGN_KEY }}
  run: |
    echo "$MINISIGN_KEY" > /tmp/minisign.key
    minisign -Sm artifact -s /tmp/minisign.key
    rm -f /tmp/minisign.key
```

The `-S` flag produces a detached `.minisig` signature file alongside the artifact.
