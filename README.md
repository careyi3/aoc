# AOC

Welcome to my AOC solutions and CLI written in Rust!

## Build & Run

You can compile this code using `cargo build --release` (`--release` is recommended for performance for some of the solutions). This will build the CLI executable.

You can also build a run the code using the built in `run` script found at `./script/run`.

This is just a wrapper around `cargo run --release <args>`.

### Inputs

While running the CLI the user is required to enter four params as follows `{year} {day} {part} {input}`

| Param | Description |
| --- | --- |
| `{year}` | year as a four letter string, `2015`, `2016`, etc. |
| `{day}` | two letter string for the day, `01`, `02` .. `24`, `25` |
| `{part}` | number, either `1` or `2` |
| `{input}` | a string that must match a file name in the solutions input directory (usually `input` or `sample` for me) |

so, `./script/run 2015 01 1 input` will execute the part 1 of the solution for year 2015 day 1 with the puzzle input being pulled from the file named `input` in the corrisponding inputs directory.

_Note: Because of the requirement that inputs be kept secret from the creator of AOC, I have not provided my inputs or answers in this repo (you should also do the same!). As such, there is a directory missing from this repo that the code assumes exists. It should be a directory called `inputs` at the root of this repo. It's layout is shown below._

```md
aoc/inputs
├── 2015
│   ├── 01
│   ├── 02
│   │   ├── input
│   │   ├── sample
│   │   └── sample2
│   ├── ...
│   └── 25
└── 2016
```

## Solutions

See below for all of the supported years.

| Year | Stars |
| --- | --- |
| [2015](./y2015/solutions) | 47 |
| [2016](./y2016/solutions) | 47 |
| [2017](./y2017/solutions) | 46 |
| [2018](./y2018/solutions) | 6 |

## TODOs

I haven't been able to solve every day of every year, the following is a list for me to go back to attempt to solve days I couldn't solve and so I skipped.

| Year | Day | Part |
| --- | --- | --- |
| 2015 | 19 | 1 & 2 |
| 2016 | 11 | 1 & 2 |
| 2017 | 21 | 1 & 2 |
| 2017 | 23 | 2 |
