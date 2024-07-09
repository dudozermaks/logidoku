# 🔢 LogiDoku: A Sudoku library
LogiDoku is a Rust library for solving and grading Sudoku puzzles using **logical methods**. It is designed to mimic the way a human would solve a Sudoku puzzle, using techniques such as **Naked Singles**, **Hidden Singles**, and other logical deductions, listed below.

## 🚀 Example
TODO example
```rust
assert_eq!(2+2, 4);
```

## 📋 Available methods
You can find description for all methods, used in this solver at sudokuwiki.org.
Here is the list of currently supported methods:

1. Naked Single/Pair/Triple/Quad
1. Hidden Single/Pair/Triple/Quad
1. Pointing Pairs/Triples
1. Box Line Reduction (I split it into Pair and Triple versions)
1. All the fishes: X-Wing, Swordfish, Jellyfish
1. Simple Coloring

In total (counting every variation of every method) LogiDoku can use **16** methods.
