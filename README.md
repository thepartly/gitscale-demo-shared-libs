# gitscale-demo-shared-libs

The crates the [GitScale demo](https://github.com/thepartly/gitscale-demo)'s
services share: `logging` (tracing setup: service name, level, pretty or
JSON) and `hello` (`greet(name, style)`). The services depend on them by
path, at `imports/shared-libs/crates/*`.
