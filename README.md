# silly-quine
A very inefficient way to produce prime implicant tables. Many optimisations can be made and maybe I will implement them in the future.

It uses a very silly implementation of the "Quine-McCluskey" algorithm to generate and display a prime implicant table from a set of minimum terms. You can also display some of the intermediate steps of the algorithm.

**!! This is not intended for any sort of practical use, it's just for fun and I do not plan to really do much with it, even though I am aware of some potential issues**

## Example:

```rust
use silly_quine::{BinaryFunction, MintermTable, ImplicantTable};

fn main() {
    let bin_f = BinaryFunction(vec![2,6,8,9,10,11,14,15]);
    let min_t = MintermTable::from(bin_f).reduce_all();
    println!("{}", min_t);
    let imp_t = ImplicantTable::from(min_t);
    println!("{}", imp_t);
}
```
Output:
```
+--------------+------------+
| Min Values   | Implicants |
+===========================+
| 2 6 10 14    | _ _ 1 0    |
|--------------+------------|
| 8 9 10 11    | 1 0 _ _    |
|--------------+------------|
| 10 11 14 15  | 1 _ 1 _    |
+--------------+------------+

+----------+---+---+---+---+----+----+----+----+
| X        | 2 | 6 | 8 | 9 | 10 | 11 | 14 | 15 |
+==============================================+
| _ _ 1 0  | Y | Y |   |   | Y  |    | Y  |    |
|----------+---+---+---+---+----+----+----+----|
| 1 0 _ _  |   |   | Y | Y | Y  | Y  |    |    |
|----------+---+---+---+---+----+----+----+----|
| 1 _ 1 _  |   |   |   |   | Y  | Y  | Y  | Y  |
+----------+---+---+---+---+----+----+----+----+
```
