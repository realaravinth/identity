[![codecov](https://codecov.io/gh/shuttlecraft/auth-backend/branch/master/graph/badge.svg)](https://codecov.io/gh/shuttlecraft/auth-backend)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
[![dependency status](https://deps.rs/repo/github/shuttlecraft/auth-backend/status.svg)](https://deps.rs/repo/github/shuttlecraft/auth-backend)

# Authentication Backend

**STATUS: ACTIVE DEVELOPMENT**

## How to build

* Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Clone the repository with:

```
$ git clone https://github.com/shuttlecraft/auth-backend
```

* Build with Cargo:

``` 
$ cd auth-backend && cargo build
```

## Setup

* **Database:**
At the moment, we have support one PostgreSQL, go into
`config/default.toml` and configure the database.

* **Server:**
The server is configurable up to some extent, please see
[default.toml](./config/default.toml) for more details
