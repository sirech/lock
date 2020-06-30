#!/usr/bin/env bats

@test "prints help" {
  lock -h | grep 'acquiring a file lock'
}

@test "prints a version number" {
  lock --version | grep 'lock [0-9]\.[0-9]\.[0-9]'
}

@test "fails on an invalid option" {
  run lock -x

  [ "$status" -eq 1 ]
}

@test "outputs the lockfile being used" {
  lockfile=/tmp/lock.$$
  cmd='echo hello'
  run lock -c "$cmd" -l $lockfile

  [ "$status" -eq 0 ]
  [[ "${lines[0]}" =~ "$lockfile" ]]
}

@test "outputs the command being run" {
  lockfile=/tmp/lock.$$
  cmd='echo hello'
  run lock -c "$cmd" -l $lockfile

  [ "$status" -eq 0 ]
  [[ "${lines[1]}" =~ $cmd ]]
}

@test "writes down the command and timestamp in the lock file" {
  lockfile=/tmp/lock.$$
  cmd='echo hello'
  run lock -c "$cmd" -l $lockfile

  cat $lockfile | grep "command\[$cmd\]"
}
