#!/bin/sh

DAY=$(printf '%02d' $1)
TARGET=src/days/day$DAY.rs

if [ -f "$TARGET" ]; then
  echo "Error: $TARGET already exists."
  exit 1
fi

cp src/days/template.rs $TARGET
sed -i s,DayNN,Day$DAY,gi $TARGET
echo "pub mod day$DAY;" >> src/days/mod.rs
