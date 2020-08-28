[![Build Status](https://travis-ci.com/shuttlecraft/auth-backend.svg?branch=master)](https://travis-ci.com/shuttlecraft/auth-backend)
[![License:GPLv2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
# Authentication Backend

**STATUS: ACTIVE DEVELOPMENT**

## How to build

* Install Cargo using [rustup](https://rustup.rs/) with:

`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

* Clone the repository with:

`$ git clone https://github.com/shuttlecraft/auth-backend`

* Build with Cargo:

` $ cd auth-backend && cargo build`

## Setup

* **Database:**
At the moment, we have support one PostgreSQL, go into
`config/default.toml` and configure the database.

* **Server:**
The server is configurable up to some extent, please see
[default.toml](./config/default.toml) for more details
