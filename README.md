# SigmaOS Thingy

Small Rust library and binary for registering desktop quick actions. `Thingy`
provides in-memory action lookup and registration, while `Action` stores a
label, optional icon name, and executable command. The built-in actions name
the lock screen, settings, terminal, and file manager programs.

## Process execution

Action activation now starts an executable directly with `std::process::Command`;
it does not invoke a shell. The command string is split on whitespace, so shell
quotes and expansions are not interpreted. A `true` result from `activate`
means the OS accepted the spawn request, not that the child completed
successfully. Callers should register trusted executable names and arguments.

## Service status

The D-Bus service and event loop are not implemented. `Thingy::run()` returns
`Unsupported`, and the current binary reports that error and exits nonzero
instead of appearing to start a working service. There is no persistent action
store or graphical panel backend yet.

## Development

The crate uses the Rust standard library and `log`. CI runs
`cargo test --all-targets --locked`; check the workflow result for the commit
under review before claiming the build passed.
