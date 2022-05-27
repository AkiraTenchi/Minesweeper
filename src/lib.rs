use std::collections::HashSet;

type Position = (usize, usize);

struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<usize, usize>,

}
