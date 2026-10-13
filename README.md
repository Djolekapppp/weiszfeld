# Weiszfeld

A Rust implementation of Weiszfeld's algorithm for solving the weighted Weber problem in two dimensions.

The program finds a location $(x,y)$ that minimizes the weighted sum of Euclidean distances to a set of points. It supports configurable convergence tolerance and a maximum number of iterations, and reports the resulting location, objective function value, iteration count, and execution time.

## The Weber Problem

Given $n$ points with coordinates $(x_i,y_i)$ and positive weights $w_i$, the goal is to find a point $(x^*,y^*)$ minimizing

$$
F(x,y)=\sum_{i=1}^{n}w_i\sqrt{(x-x_i)^2+(y-y_i)^2}.
$$

This problem has applications in facility location, logistics, and operations research.

The implementation uses Weiszfeld's iterative algorithm and checks whether an input point is already an optimal solution before starting the iterations.

## Features

- Reads point coordinates and weights from `.dat` files.
- Implements Weiszfeld's iterative algorithm.
- Supports configurable convergence tolerance (`epsilon`).
- Supports a maximum iteration limit.
- Checks whether an input point is an optimal solution.
- Reports the computed location, objective function value, iteration count, and execution time.

## Requirements

- Rust
- Cargo

No external Rust dependencies are currently required.

## Getting Started

Clone the repository:

```
git clone https://github.com/Djolekapppp/weiszfeld.git
cd weiszfeld
````

Build the project in release mode:

```
cargo build --release
```

Run the program with a sample dataset:

```
cargo run --release -- data/weber0.dat
```

## Command-Line Arguments

The input file path is required. Algorithm parameters are optional.

| Argument | Description | Default |
| --- | --- | --- |
| `FILE` | Path to the input `.dat` file | Required |
| `--epsilon VALUE` | Convergence tolerance | `0.01` |
| `--max-iter N` | Maximum number of iterations | `100` |

## Examples

Run with default parameters:

```
cargo run --release -- data/weber0.dat
```

Specify a convergence tolerance:

```
cargo run --release -- data/weber0.dat --epsilon 0.001
```

Set both the tolerance and maximum iteration count:

```
cargo run --release -- data/weber0.dat --epsilon 0.001 --max-iter 10000
```

The maximum iteration limit prevents the algorithm from running indefinitely.

## Input File Format

The program reads `.dat` files containing point identifiers, coordinates, and weights.

The expected structure is:

```
param:
    I: a b w :=
    A   3   7   80
    B   8   1   30
    C  10   5   25
    D  12   1   40
;
```

Each data row must contain four whitespace-separated fields:

```
LABEL X_COORDINATE Y_COORDINATE WEIGHT
```

For example:

```
A 3 7 80
B 8 1 30
C 10 5 25
```

- `param:` marks the beginning of the parameter declaration.
- `I: a b w :=` defines the table columns and must immediately follow the `param:` line.
- `LABEL` identifies a point. The identifier is not used in the optimization.
- `X_COORDINATE` and `Y_COORDINATE` specify the point's location.
- `WEIGHT` specifies the point's positive weight.
- `;` terminates the data table.

A sample dataset is available in `data/weber0.dat`.

## Output

The program reports the input file, number of loaded points, algorithm parameters, iteration count, termination status, computed location, objective function value, and execution times.

Example output:

```
========== Weber Problem ==========
Input file:       data/weber0.dat
Points loaded:    4
Epsilon:          0.01
Max iterations:   100
Iterations:       12
Status:           Converged
-----------------------------------
Optimal point:    (6.985968, 4.302673)
Objective value:  806.174740
-----------------------------------
File loading:     0.120 ms
Algorithm time:   0.040 ms
Total time:       0.180 ms
===================================
```

The numerical values and timings are illustrative and may differ depending on the input dataset and machine.

Possible termination outcomes include:

- Converged: The convergence criterion was satisfied.
- Maximum iterations reached: The iteration limit was reached before satisfying the convergence criterion.
- Optimal input point found: An optimal solution was detected among the input points before the iterative procedure started.

## Project Structure

```
weiszfeld/
├── data/
│   └── weber0.dat
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

## License

No license has been specified yet. If you intend others to use, modify, or redistribute this project, consider adding an appropriate open-source license.
