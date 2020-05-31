#!/usr/bin/env bats

infile="$BATS_TEST_DIRNAME/raw.ipynb"
reffile="$BATS_TEST_DIRNAME/stripped.ipynb"

cargo build -q >&2

@test "pipe in pipe out" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cat $infile | cargo run -q > $outfile

  echo "" >> $outfile

  diff -u $outfile $reffile
}

@test "file in pipe out" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cargo run -q $infile > $outfile

  echo "" >> $outfile

  diff -u $outfile $reffile
}

@test "file in file out" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cargo run -q $infile $outfile
  echo "" >> $outfile
  diff -u $outfile $reffile

  # make sure it works when the file exists
  cargo run -q $infile $outfile
  echo "" >> $outfile
  diff -u $outfile $reffile
}

@test "is idempotent" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cargo run -q $infile $outfile
  echo "" >> $outfile
  diff -u $outfile $reffile

  # make sure it doesn't change when running again on same file
  outfile2="$tmpdir/out2.ipynb"
  cargo run -q $outfile $outfile2
  echo "" >> $outfile2
  diff -u $outfile $outfile2

}