# ARSY Installer and Onboarding Specification

**Target UX:** a simple, guided, one-command self-hosted installation inspired by the low-friction onboarding of tools such as Coolify.

## 1. When does the installer ship?

- **Phase 0:** local developer bootstrap only (scripts/dev-up.sh).
- **Phase 1:** configuration and environment validation conventions.
- **Phase 2:** production single-server installer, after the API, worker, database migrations, release images, health checks, and agent/runtime contracts are deployable together.

Do not publish a production installer URL before a tagged, tested release exists.

## 2. Desired first-run experience

1. Create a clean supported Linux server.
2. Connect over SSH as root or a sudo-capable administrator.
3. Run the installer command.
4. Review preflight findings and confirm changes.
5. Wait for services to become healthy.
6. Open the URL printed by the installer.
7. Complete a one-time, secure admin bootstrap.
8. Save the generated recovery/backup information.

Recommended safer command (download, inspect, then execute):

~~~bash
curl -fsSL https://arsy.cloud/install.sh -o arsy-install.sh
less arsy-install.sh
sudo bash arsy-install.sh
~~~

The short pipe-to-shell form may be offered only after the script is public, release-pinned or integrity-verifiable, and the trust model is documented. Prefer a versioned release URL plus checksum/signature verification for production.

## 3. Phase 2 installer responsibilities

### Preflight
- Detect Linux distribution/version and CPU architecture.
- Check root/sudo privileges.
- Check RAM, CPU, free disk, DNS, clock synchronization, and required ports.
- Detect existing Docker/container runtime and conflicting services.
- Detect whether the target is a supported clean installation.
- Explain every required change before applying it.

### Installation
- Install only documented dependencies using supported vendor repositories.
- Create a dedicated ARSY directory and service account where practical.
- Create persistent data directories with restrictive ownership and permissions.
- Generate unique secrets using a cryptographically secure random source.
- Write a root-readable environment file (for example mode 0600).
- Fetch version-pinned images and verify release digests/signatures.
- Configure a supported reverse proxy and HTTPS path.
- Run database migrations exactly once with a controlled release procedure.
- Start control plane and worker.
- Wait for readiness checks and print actionable diagnostics on failure.

### Bootstrap
- Do not expose an open public registration endpoint that lets the first internet visitor claim administrator privileges.
- Use a one-time setup token or a local-only bootstrap flow.
- Expire and invalidate bootstrap credentials after account creation.
- Require MFA setup or strongly encourage it before public exposure.

### Operations
- arsy upgrade or equivalent versioned upgrade procedure.
- Backup database, persistent data, and encryption/configuration secrets.
- Restore workflow with validation.
- Safe uninstall that preserves data unless destructive removal is explicitly confirmed.
- Logs and diagnostics command.
- Installer version and installed release version reporting.

## 4. Supported target policy

Start narrowly. A proposed first support matrix is Ubuntu LTS on amd64 and arm64 after CI and real-host testing. Add Debian or other distributions only after testing package installation, firewall behavior, system services, and upgrade paths.

Do not silently modify an unsupported host. Fail with a clear reason and a manual installation path.

## 5. Network and security behavior

- Never disable the host firewall wholesale.
- Show the exact ports that must be opened and why.
- Keep database and internal service ports bound to loopback or private networks.
- Expose only the intended web ingress and documented agent channels.
- Do not print secret values to logs or terminal output.
- Never accept secrets through command-line arguments where process listings may expose them.
- Do not send provider credentials to the browser.
- Avoid executing remote scripts without version and integrity controls.
- Make installation idempotent and preserve user data on failed upgrades.

## 6. Installer completion output

On success, print:
- ARSY release version.
- Dashboard URL.
- Bootstrap instructions (not the secret itself).
- Service health summary.
- Data and configuration backup locations.
- How to view logs and run diagnostics.
- Upgrade and uninstall documentation links.

On failure, print:
- Failed phase.
- Non-secret diagnostic summary.
- Log file location.
- Whether rollback succeeded.
- Safe next action.

## 7. Acceptance tests

- Fresh supported OS install.
- Unsupported OS rejection.
- Non-root invocation behavior.
- Interrupted download and interrupted package installation.
- Existing Docker/runtime detection.
- Re-running installer (idempotency).
- Secret file permission checks.
- Port conflict detection.
- Failed migration recovery.
- Service readiness timeout.
- Upgrade from previous release.
- Backup and restore on a clean test host.
- Uninstall preserving data unless explicitly asked to delete it.

## 8. Current status

This document is a specification. It is not a claim that the production installer already exists. The first repository scripts prepare local development only.