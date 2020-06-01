#!/usr/bin/env bats

infile="$BATS_TEST_DIRNAME/raw.ipynb"
reffile="$BATS_TEST_DIRNAME/stripped.ipynb"

@test "pipe in pipe out" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cat $infile | cargo run -q > $outfile
  status=$?
  [ "$status" -eq 0 ]

  echo "" >> $outfile

  diff -u $outfile $reffile
}

@test "file in pipe out" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  cargo run -q $infile > $outfile
  status=$?
  [ "$status" -eq 0 ]

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
  run cargo run -q $infile $outfile
  [ "$status" -eq 0 ]
  echo "" >> $outfile
  diff -u $outfile $reffile
}

@test "is idempotent" {
  tmpdir=$(mktemp -d)
  outfile="$tmpdir/out.ipynb"

  run cargo run -q $infile $outfile
  [ "$status" -eq 0 ]
  echo "" >> $outfile
  diff -u $outfile $reffile

  # make sure it doesn't change when running again on same file
  outfile2="$tmpdir/out2.ipynb"
  run cargo run -q $outfile $outfile2
  [ "$status" -eq 0 ]
  echo "" >> $outfile2
  diff -u $outfile $outfile2

}

@test "input file does not exist" {
  infile="nopedinopenope.ipynb"
  
  run cargo run -q $infile
  [ "$status" -ne 0 ]
  [[ "$output" == *"No such file or directory"* ]]
}

@test "invalid json input" {
  infile=$BATS_TMPDIR"/infile.ipynb"
  echo "{balbal ]" > $infile

  run cargo run -q $infile
  [ "$status" -ne 0 ] 
  [[ "$output" == *"Error parsing JSON"* ]]
}