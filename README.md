# is-semantic-release-configured

[![Build Status]](https://github.com/EricCrosson/is-semantic-release-configured/actions/workflows/release.yml)

[build status]: https://github.com/EricCrosson/is-semantic-release-configured/actions/workflows/release.yml/badge.svg?event=push

**is-semantic-release-configured** is a small binary to test if [semantic-release] is configured.

This can be used to manage control flow in shell scripts.

> **Note**: this tool currently only checks for the existence of configuration,
> it does not validate the content. This tool does not care if your
> configuration will be rejected by semantic-release.

[semantic-release]: https://github.com/semantic-release/semantic-release
