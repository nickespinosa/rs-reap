# Security Policy

## Supported versions

Security fixes are applied to the latest release on the default branch.

## Reporting a vulnerability

Please open a private security advisory on GitHub when possible:

https://github.com/nickespinosa/rs-reap/security/advisories/new

If that is unavailable, contact the maintainer through GitHub. Do not file a
public issue for undisclosed vulnerabilities.

## Scope notes

`rs-reap` runs privileged process-wait syscalls on Unix. Reports involving
incorrect wait behavior, lock races that steal child exit status, or unsafe
blocks without adequate justification are in scope.
