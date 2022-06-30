mod life;
mod render;
use crate::life::{update_cells, Board};

use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};
use std::{
    io::{stdout, Write},
    process::Command,
};

const ALIVE: char = '■';
const DEAD: char = '□';

fn render(board: &Board) -> String {
    board
        .cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    match cell {
                      0 => String::from(DEAD),
                      1 => String::from(ALIVE),
                      _ => String::from('_'),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() -> Result<()> {
    let mut board = Board::new(10, 10);

    board.cells[3][5] = 1;
    board.cells[4][5] = 1;
    board.cells[4][3] = 1;
    board.cells[4][6] = 1;

    // TODO: Some nice user input here please
    while true {
        execute!(stdout(), Clear(ClearType::All),)?;
        stdout()
            .execute(Print(render(&board)))?
            .execute(ResetColor)?;

        board = update_cells(&board);

        // TODO: Let's not really do this
        let mut child = Command::new("sleep").arg("1").spawn().unwrap();
        let _result = child.wait().unwrap();
    }

    // or using functions

    Ok(())
}
