<div align="center">

  <h1><code>cards-client-rs</code></h1>

  <strong>A superb cards game web app to play with friends.</strong>
  <br>
  <i>At least that's what we are trying to achieve.</i>

  <p>
    <a href="https://travis-ci.org/totorigolo/cards-client-rs">
      <img src="https://api.travis-ci.com/totorigolo/cards-client-rs.svg?branch=master" alt="Build Status" />
    </a>
  </p>

  <h3>
    <a href="https://cards.busy.ovh/">Play online</a>
    <span> | </span>
    <a href="#">Dev corner</a>
  </h3>

  <sub>Built in Rust 🦀 for the Web 🕸, using [Yew](https://yew.rs/).</sub>
</div>

## About

TBD

## 🚴 Usage

If you just want to play just head to our [web site](https://cards.busy.ovh/).
If instead you enjoy playing with code, or you want to give us a hand, keep
reading.

### 🐑 Prerequisites

1. Install Rust using [rustup](https://www.rust-lang.org/tools/install).
2. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
3. Install [Node.js](https://nodejs.org/) (for `npm`).
4. Install [Caddy](https://caddyserver.com/docs/download), for reverse proxy.
   * We recommend using the version from the official distribution.
   * Be careful to install Caddy v2.

### 🛠️ How to build

```bash
# Needed only the first time
npm install

# To build, use one of:
npm run build:dev
npm run build:prod

# To run the tests
npm run test:rs

# To build and start a web server, watching for changes.
# Afterwards, go to: http://localhost:42803/.
# However, this won't serve the API. Read below for how-to.
npm start
```

The instructions above are for the frontend only. If you want the full website,
i.e. with the API, you need to run the game server and serve it at `/api`.

To simplify this setup, there is a `Caddyfile` in this repository that comes
pre-configured. Just make sure to respect the ports explained in the file.

```bash
# To start the reverse proxy
caddy run --watch

# Or to start as a daemon
caddy start --watch
caddy stop
```
