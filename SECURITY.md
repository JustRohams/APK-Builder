# Security Policy

## Supported versions

| Version | Supported |
| --- | --- |
| 0.1.x | ✅ |

## Reporting a vulnerability

Please **do not** open a public issue for security problems.
Use GitHub's private
[security advisory](https://github.com/JustRohams/APK-Builder/security/advisories/new)
form instead. You will get a response as soon as possible.

## Notes

- APKs produced by this tool are **debug-signed** and meant for testing and
  personal use.
- The tool runs `gradle` on a generated project in your temp directory; only
  build content you trust.
