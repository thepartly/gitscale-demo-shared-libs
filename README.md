# gitscale-demo-shared-libs

The crates the [GitScale demo](https://github.com/thepartly/gitscale-demo)'s
services share: `logging` (tracing setup: service name, level, pretty or
JSON) and `hello` (`greet(name, style)`). The services depend on them by
path, at `imports/shared-libs/crates/*`.

## Crates

### `hello`

The greeting every service gives: `greet(name, style)` returns
`Hello, <name>!`, as is with `Style::Plain`, upper case with `Style::Shout`.

```rust
assert_eq!(hello::greet("Ada", hello::Style::Shout), "HELLO, ADA!");
```

### `logging`

Tracing setup, one call at a service's start: `init(service, format)`
installs the global subscriber and logs `logging started` with the service's
name. The level comes from `RUST_LOG`, `info` when unset. `Format::Pretty` is
for a person reading a terminal, `Format::Json` one object per line for a log
pipeline.

```rust
logging::init("application-a", logging::Format::Pretty);
```

## Who uses them

| Service | Greeting | Logs |
|---|---|---|
| application-a | `Style::Plain` | `Format::Pretty` |
| application-b | `Style::Shout` | `Format::Json` |

Each asks for its own release in its `.gitscale.toml`, and they need not
agree: application-b asks for an older one than application-a, and in a
checkout of both the higher request wins (`git explain imports/shared-libs`).
frontend never sees these crates: it takes the applications' SDKs only.

## In the demo

shared-libs is the bottom of the graph, so a change here is the one that
travels furthest: topics 3 to 7 of the demo start here and show which
dependants join the topic, which stay at their release, and how a pin is
raised. During a topic, an edit to `greet` restarts application-a in the
local stack, and the page shows it.

## Working on it

```sh
cargo test    # both crates, from the root of this repository
```

Releases are calendar versions, `v1-YYYY.MM.DD-hhmmss`, one per merge to
`main`; the services move to one with `git upgrade`.
