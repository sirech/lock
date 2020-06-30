# lock

This is a CLI tool that allows to run a command after acquiring a file lock, allowing to coordinate the execution of multiple commands that call `lock` independently.

## Why?

It's born out of [CircleCI](https://circleci.com/) running multiple pipelines at the same time due to [Dependabot](https://dependabot.com/) updates. This in the end would cause `docker-compose` to be run multiple times in parallel, which seems to cause some timeout errors. Now, everybody should wait for their turn.

## How to build it

Run `./go` to see all the actions that can be performed.

### Building a new release

Inspired by [this](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/) guide, I'm cross compiling this from OSX to linux using `musl`. Two extra libraries are needed before build:

```shell
# Build linux binaries from OSX
rustup target add x86_64-unknown-linux-musl

# Make smaller binaries
brew install upx
```

Then run `./go build-release` to build a binary.

## User guide

### Installation

Download the [latest binary release](./releases) and put it on your _PATH_.

### Usage

Run `lock -h` to see the different options.
