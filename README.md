# samplr

**samplr** is a CLI tool to randomly sample data; generating a fixed size sample of input lines with uniform probabilities.

## Installation

### Source

> Requires [Rust](https://www.rust-lang.org/) to be installed.

```
git clone https://github.com/SteadBytes/sample.git
cd sample
cargo install --path .
```

## Examples

Sample 15 lines from a file:

```
sample -n 15 things.txt
```

Sample 15 lines from standard input:

```
<things.txt | sample -n 15
```
 
Sample 15 lines from multiple files:

```
sample -n 15 things.txt other_things.txt
```

## Sampling Algorithm

**samplr** uses a Reservoir Sampling algorithm to generate fixed size samples from an input stream of unknown length. For more details, see [the implementation](./src/lib.rs) and the linked [blog article](https://steadbytes.com/blog/reservoir-sampling).

