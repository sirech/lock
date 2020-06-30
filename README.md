# lock

This is a CLI tool that allows to run a command after acquiring a file lock, allowing to coordinate the execution of multiple commands that call `lock` independently.

## Why?

This is born out of [CircleCI](https://circleci.com/) running multiple pipelines at the same time due to [Dependabot](https://dependabot.com/) updates. This in the end would cause `docker-compose` to be run multiple times in parallel, which seems to cause some timeout errors. This way, everybody should wait their turn.

## How to build it

Run `./go` to see how to check requirements, run tests, or build the binary.

## How to run it

Run `lock -h` to see the different options.
