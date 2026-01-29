### ⚠️ Repo moved to Codeberg: https://codeberg.org/bnz/tgrep ⚠️

---
---

# tgrep — grep but with time

`tgrep` is a much simpler version of `grep` but with a precise scope in mind:
print, for each line matched, the delta of time between that line and the previous one.

Why? Personally I use it quite often when I'm debugging a performance related bug.

With `tgrep` I can easily find where a piece of code took a lot of time or see if some output took much more that others to compute, and that by simply add a couple of logs.

### A couple of notice:
- **This is not a benchmark tool**. It's not precise enough and make sense to use it only for debug purposes.
- Do not forget that stdout in a pipe will be buffering. Sometimes you have to [turn it off](https://unix.stackexchange.com/questions/25372/turn-off-buffering-in-pipe).

## Example
```console
$ ./a.out < recipes.txt | tgrep "HALP"
0ns     | starting HALP
148ns   | HALP processing 'coniglio alla ligure'
16us    | HALP processing 'coniglio alla ligure' end
98ns    | HALP processing 'focaccia'
9us     | HALP processing 'focaccia' end
118us   | HALP processing 'pesto con le noci'
69ms    | HALP processing 'pesto con le noci' end
154us   | HALP processing 'torta di riso'
7us     | HALP processing 'torta di riso' end
```

## Installation
```console
$ cargo install --git https://github.com/Bnz-0/tgrep
```

## Usage
```console
$ tgrep -h
Grep but with time

Usage: tgrep [OPTIONS] [PATTERN]

Arguments:
  [PATTERN]

Options:
  -i, --ignore-case           Ignore case distinctions in patterns and data
  -u, --fix-unit <TIME_UNIT>  Fix the unit time used while printing [possible values: ns, us, ms, s]
  -h, --help                  Print help
  -V, --version               Print version
```

