# rs-reap

A small Rust library for reaping abandoned child processes in PID 1-style
applications, including containers. Supported Unix targets use `SIGCHLD` and
nonblocking `wait4`; Windows and Solaris provide a safe no-op implementation.

See the [API documentation](https://docs.rs/rs-reap) for usage.

## Development

```bash
make verify
make ci
make clean
```

Commits and pull requests follow [Conventional Commits](https://www.conventionalcommits.org/).
Release Please uses those commits to manage Semantic Versioning releases.

## License

MPL-2.0. See [LICENSE](LICENSE).
