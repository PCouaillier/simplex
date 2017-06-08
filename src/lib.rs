extern crate reduce;
use reduce::Reduce;
use std::error::Error;

#[cfg(test)]
mod tests {
    use ::{simplex, Board, Equation};
    #[test]
    fn it_works() {
        assert_eq!(simplex(Board {
            eqs: vec![
                Equation::new(vec![3,2], 1800),
                Equation::new(vec![1,0], 400),
                Equation::new(vec![0,1], 600)
            ],
            func_eco: Equation::new(vec![30,50], 0)
        }).unwrap().1,-36000);
    }
}

#[derive(Clone, Debug)]
pub struct Equation {
    multiplicateurs: Vec<isize>,
    res: isize
}

impl Equation {
    pub fn new(multiplicateurs: Vec<isize>, res: isize) -> Equation {
        Equation { multiplicateurs: multiplicateurs, res: res }
    }
}

#[derive(Debug)]
pub struct Board {
    pub eqs: Vec<Equation>,
    pub func_eco: Equation,
}

#[derive(Debug)]
struct Cell {
    val: isize,
    row: usize,
    column: usize
}

impl Cell {
    fn new(val: isize, row: usize, column: usize) -> Cell {
        Cell {val: val, row: row, column: column }
    }
}

// (max, column)
fn max_in_eco(multiplicateurs: &[isize]) -> usize {
    multiplicateurs.into_iter().enumerate().reduce(
        |acc, (i, v)|
            if acc.1 < v {
                (i, v)
            } else {
                acc
            }).unwrap().0
}

fn divide_res(a: isize, b: isize) -> isize {
    if b != 0 {
        a/b
    } else {
        isize::max_value()
    }
}

// (min, row)
fn min_in_res(eq: &[Equation], column: usize) -> usize {
    eq.iter().map(|e|divide_res(e.res,e.multiplicateurs[column])).enumerate().reduce(
        |acc, (i,v)|
            if v < acc.1 {
                (i, v)
            } else {
                acc
            }
    ).unwrap().0
}

fn generate_new_board(board: &Board, pivot: &Cell) -> Board {

    let mut new_board = Board {eqs:Vec::new(), func_eco:Equation{multiplicateurs: Vec::with_capacity(6), res:0}};

    let col_pivot: Vec<isize> = board.eqs.iter().map(|e|e.multiplicateurs[pivot.column]).collect();

    let ligne_pivot: &Equation = &board.eqs[pivot.row];

    for (row, eq) in board.eqs.iter().enumerate() {
        new_board.eqs.push(apply_row(eq, col_pivot[row], pivot.val, ligne_pivot));
    }
    new_board.func_eco = apply_row(&board.func_eco, board.func_eco.multiplicateurs[pivot.column], pivot.val, ligne_pivot);

    new_board
}

fn apply_row(eq: &Equation, a_i: isize, pivot: isize, ligne_pivot: &Equation) -> Equation {

    let mut equations = Vec::with_capacity(6);

    for (col, multiplicateur) in eq.multiplicateurs.iter().enumerate() {
        equations.push(multiplicateur - ((a_i/pivot)*ligne_pivot.multiplicateurs[col]));
    }
    Equation {
        multiplicateurs: equations,
        res: eq.res - ((a_i/pivot)*ligne_pivot.res)
    }
}

fn need_iterate(eq: &[isize]) -> bool {
    eq.into_iter().any(|a|*a>0)
}

fn add_zeros(board: &mut Board) {
    let size = board.eqs.len();
    let mut i = 0;
    for mut eq in &mut board.eqs {
        for _ in 0..size {
            eq.multiplicateurs.push(0);
        }
        eq.multiplicateurs[size+i] = 1;
        i+=1;
    }
    for _ in 0..size {
        board.func_eco.multiplicateurs.push(0);
    }
}

fn print_board(board: &Board) {
    for eq in &board.eqs {
        println!("{:?}", &eq);
    }
    println!("{:?}", &board.func_eco);
}

pub fn simplex(input_board: Board) -> Result<(Vec<isize>, isize), Box<Error>> {

    let mut board: Board = input_board;

    //let original_len = board.func_eco.multiplicateurs.len();

    add_zeros(&mut board);

    println!("");
    print_board(&board);

    while {

        let pivo;
        {
            let max_column = max_in_eco(board.func_eco.multiplicateurs.as_slice());

            let min_row = min_in_res(board.eqs.as_slice(), max_column);

            pivo = Cell::new(board.eqs[min_row].multiplicateurs[max_column], min_row, max_column);
        }


        println!("");
        println!("{:?}", pivo);

        board = generate_new_board(&board, &pivo);

        print_board(&board);

        need_iterate(board.func_eco.multiplicateurs.as_slice())
    }{}
    Ok((board.func_eco.multiplicateurs, board.func_eco.res))
}
