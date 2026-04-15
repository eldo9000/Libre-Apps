# Key Rotation Policy

## Active Keys

| Key file | Key ID | Purpose | Valid from |
|----------|--------|---------|------------|
| `librewin-releases.pub` | `0531C1E41A7F05AF` | Production tag releases | 2026-04-14 |
| `librewin-releases-dev.pub` | `676C01F6F2C480D5` | Non-tag CI builds | 2026-04-14 |

## GitHub Actions Secrets

| Secret name | Key |
|-------------|-----|
| `MINISIGN_KEY` | Private key for `librewin-releases.pub` |
| `MINISIGN_KEY_DEV` | Private key for `librewin-releases-dev.pub` |

The private keys are stored **only** in GitHub Actions secrets. They are not in this repo,
not in any backup, not in any developer's home directory. The `/tmp` keygen artifacts
were deleted immediately after the secrets were set.

## When to rotate

- Suspected private key compromise
- Developer with key access leaves the team
- Routine rotation: annually, before a major version release

## How to rotate

1. Generate a new keypair on an air-gapped or trusted machine:
   ```
   minisign -G -p librewin-releases-new.pub -s librewin-releases-new.key -W \
     -c "LibreWin OS release signing key"
   ```

2. Update the GitHub Actions secret (`MINISIGN_KEY` or `MINISIGN_KEY_DEV`) with the new
   private key content. Delete the local `.key` file immediately after.

3. Replace the public key file in this repo (`keys/librewin-releases.pub` or
   `keys/librewin-releases-dev.pub`) with the new `.pub` file.

4. Update the table above: add a row for the new key, move the old key to
   the "Retired Keys" section below with its last signed tag.

5. Open a PR in `eldo9000/LibreWin-OS` to update the baked-in public key in the
   install manager. Until that PR merges, old installs cannot verify new releases —
   coordinate timing with an OS build.

6. Tag the first release signed with the new key. Announce the rotation in release notes.

## Retired Keys

_None yet._

## LibreWin-OS integration

The production public key (`librewin-releases.pub`) is baked into the LibreWin OS install
manager at build time. See `LibreWin-OS/build/fetch-apps.sh` and
`LibreWin-OS/build/modules/install-manager.sh` for where the key is embedded.

The dev key (`librewin-releases-dev.pub`) is never baked into shipped OS images — it is
only used by developers verifying CI artifacts manually.
