#!/bin/bash

. .env
day_id=$((1 + $day))
session_cookie=$cookie
fmt_day=$(printf "%02d" $day_id)
new_file=src/y$year/day$fmt_day.rs
year_file=src/y$year/mod.rs
cp template $new_file

sed -i "s/Day01/Day$fmt_day/" $new_file
sed -i "s#// mod day$fmt_day;#mod day$fmt_day;#" $year_file
sed -i "s#//             $day_id => Some(day$fmt_day::Day$fmt_day),#            $day_id => Some(Box::new(day$fmt_day::Day$fmt_day)),#" $year_file
sed -i "s#// $day_id,#$day_id,#" $year_file

curl -o input/y$year/d$day_id.txt --cookie "session=$session_cookie" https://adventofcode.com/$year/day/$day_id/input

sed -i "s/day=$day/day=$day_id/" .env

echo "Get the challenge for day $day_id  Here : https://adventofcode.com/$year/day/$day_id"