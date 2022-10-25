# Primes

### Prime numbers generator / checker

```bash
$ primes --check 10 11 12 13
10 is not a prime
11 is a prime
12 is not a prime
13 is a prime
```

```bash
$ primes --range 2 6
2
3
5
7
11
13
```

## Usage
```text
primes 0.1.0
Prime numbers generator / checker

USAGE:
    primes [OPTIONS]

OPTIONS:
    -c, --check <CHECK>...    Check if a number(s) is prime
    -h, --help                Print help information
    -n, --n <N>               Print first n primes [default: 0]
    -r, --range <RANGE>...    Print primes between two numbers
    -V, --version             Print version information
```
