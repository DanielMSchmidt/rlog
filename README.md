# RLog

Small and local log aggregator written in Rust.

## Usage

First create a RLog config file:

```json
{
  "backend": "./backend/log",
  "frontend": "./frontend.log"
}
```

Start rlog before you create the logs to be combined

```sh
rlog > combined.log &
RLOG_PID="$!"

# Run app / tests / whatever
kill -9 $RLOG_PID
```

The logs will look like this

```
20:42:23 backend    | started on port 1234
20:42:54 frontend   | started on port 2345
```
