
# noteguard

A high performance note filter plugin system for [strfry]

WIP!

## Usage

Filters are registered and loaded from the [noteguard.toml](noteguard.toml) config.

You can add any new filter you want by implementing the `NoteFilter` trait and registering it with noteguard via the `register_filter` method.

The `pipeline` config specifies the order in which filters are run. When the first `reject` or `shadowReject` action is hit, then the pipeline stops and returns the rejection error.

```toml
pipeline = ["protected_events", "whitelist", "ratelimit"]

[filters.ratelimit]
posts_per_minute = 8
whitelist = ["127.0.0.1"]

[filters.whitelist]
pubkeys = ["32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245"]
ips = ["127.0.0.1", "127.0.0.2"]

[filters.protected_events]
```

## Filters

You can use any of the builtin filters, or create your own!

This is the initial release, and only includes one filter so far:

### Ratelimit

* name: `ratelimit`

The ratelimit filter limits the rate at which notes are written to the relay per-ip.

Settings:

- `notes_per_minute`: the number of notes per minute which are allowed to be written per ip.

- `whitelist` *optional*: a list of IP4 or IP6 addresses that are allowed to bypass the ratelimit.

### Whitelist

* name: `whitelist`

The whitelist filter only allows notes to pass if it matches a particular pubkey or source ip:

- `pubkeys` *optional*: a list of hex public keys to let through

- `ips` *optional*: a list of ip addresses to let through

Either criteria can match

### Protected Events

See [nip70]

* name: `protected_events`

There are no config options, but an empty config entry is still needed:

`[filters.protected_events]`

## Testing

You can test your filters like so:

```sh
$ cargo build
$ <test/inputs ./target/debug/noteguard
$ ./test/delay | ./target/debug/noteguard
```

## Static builds

Static musl builds are convenient ways to package noteguard for deployment. It enables you to copy the binary directly to your server, assuming its the same architecture as the one you're building on.

```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --target x86_64-unknown-linux-musl --release
$ ldd ./target/x86_64-unknown-linux-musl/release/noteguard
	statically linked
$ scp ./target/x86_64-unknown-linux-musl/release/noteguard server:
```

[strfry]: https://github.com/hoytech/strfry
[nip70]: https://github.com/nostr-protocol/nips/blob/protected-events-tag/70.md
