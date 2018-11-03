#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"
fixture="$root/fixtures"

SUCCESSFULLY=0
WITH_FAILURE=3

(with "input from stdin"
  (when "the input is well formed"
    it "produces the expected output" && {
      WITH_SNAPSHOT="$snapshot/success-input-file-produces-correct-output" \
      expect_run ${SUCCESSFULLY} "$exe" < "$fixture/aaah.input"
    }
  )
  (when "there are unusual characters within the aaah"
    it "fails with a descriptive error" && {
      echo $'aaah\naafh' | \
      WITH_SNAPSHOT="$snapshot/failure-input-with-unusual-characters" \
      expect_run ${WITH_FAILURE} "$exe"
    }
  )
  (when "there are any characters after the aaah"
    it "fails with a descriptive error" && {
      echo $'aaaha\naah' | \
      WITH_SNAPSHOT="$snapshot/failure-input-with-characters-after-h" \
      expect_run ${WITH_FAILURE} "$exe"
    }
  )
  (when "there is no h at all"
    it "fails with a descriptive error" && {
      echo $'aaa\naah' | \
      WITH_SNAPSHOT="$snapshot/failure-input-without-any-h" \
      expect_run ${WITH_FAILURE} "$exe"
    }
  )
)
