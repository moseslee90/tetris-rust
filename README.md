# tetris-rust

A rust project to generate a tetris artificial intelligence through the genetic
algorithm

cargo run

After cargo run, follow the command line instructions:

"init-pop": generates a new population with random genes.

---

"cycle-pop": would cycle through a few generations depending on the number
specified.

---

"read-pop": reads current population and writes best individuals to file without
repopulating.

---

"next-gen": repopulates based on best individuals on file.

---

"top-play": quick demonstration of a top scoring individual playing tetris

---

Current settings make algorithm run 3 times for each individual and return the
lowest score of 3 runs. Maximum lines cleared wherby assesment for individual
ends is 700 lines Default population size is 1000 and takes a few hours to
complete.

Future optimizations would be to multi-thread the process.
