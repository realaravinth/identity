<div align="center">
<img src="assets/fingerprint.svg" alt="Wagon" width="200" height="180" />

# Identity

![CI (Linux)](https://github.com/shuttlecraft/identity/workflows/CI%20(Linux)/badge.svg)
[![codecov](https://codecov.io/gh/shuttlecraft/identity/branch/master/graph/badge.svg?token=4HjfPHCBEN)](https://codecov.io/gh/shuttlecraft/identity)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
[![dependency status](https://deps.rs/repo/github/shuttlecraft/identity/status.svg)](https://deps.rs/repo/github/shuttlecraft/identity)

### STATUS: ACTIVE DEVELOPMENT (fancy word for unusable)

</div>

**Identity** is an identity and access management platform built for the
[IndieWeb](indieweb.org)

### How to build

- Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repository with:

```
$ git clone https://github.com/shuttlecraft/authentication
```

- Build with Cargo:

```
$ cd authentication && cargo build
```

### Usage

#### Configuration:

Wagon is highly configurable.
Configuration is applied/merged in the following order:

1. `config/default.toml`
2. `config/$IDENTITY_MODE.toml`
3. environment variables.

## Setup

##### Environment variables:

| Name                            | Value                                                       |
| ------------------------------- | ----------------------------------------------------------- |
| `IDENTITY_MODE`                 | Run mode for choosing configuration(development/production) |
| `IDENTITY_SMTP_KEY`             | API key                                                     |
| `IDENTITY_DATEBASE_PASSWORD`    | Postgres password                                           |
| `IDENTITY_DATEBASE_NAME`        | Postgres database name                                      |
| `IDENTITY_DATEBASE_PORT`        | Postgres port                                               |
| `IDENTITY_DATEBASE_HOSTNAME`    | Postgres hostmane                                           |
| `IDENTITY_DATEBASE_USERNAME`    | Postgres username                                           |
| `IDENTITY_DATEBASE_POOL`        | Postgres database connection pool size                      |
| `IDENTITY_REDIS_PORT`           | Redis port                                                  |
| `IDENTITY_REDIS_HOSTNAME`       | Redis hostmane                                              |
| `IDENTITY_PORT` (or) `PORT`\*\* | The port on which you want wagon to listen to               |
| `IDENTITY_IP`                   | The IP address on which you want wagon to listen to         |

### Credits:

Logo made by [Pixel perfect](https://icon54.com/) from
[Flaticon](https://www.flaticon.com). Do check them out!
