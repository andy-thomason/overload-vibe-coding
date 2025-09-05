# Vibe coding in Rust - lessons learned

I've been writing code since the '70s and things have gone through
a number of changes since then, to say the least. From machine code
to assembly to early compilers, coding standards and practices up
to the present day where a new paradigm is emerging in the form of
LLM models focused on generating and analysing code.

All of this has been change, relentless change, and you probably should
not be in the software business if you are not quick to adapt to
the pace of technology.

As a certified old fart, I was very sceptical of the idea of LLMs
generating code and to a degree I remain so. Computational linguistics
was very definitely a thing when I started my career, we founded the
Manchester Aristotelian Society in the '80s to discuss philosophy
with a group of computational linguists who were using Prolog to
translate to and from Japanese.

Using Prolog for language was a highly Asimovian aproach to the problem.
Like the positronic brain, every detail needed to be worked out by hand, every rule of grammar
raked over and converted to statements. The idea of doing this with
neural networks would have seemed crazy as the memory and computational
requirements would have been impossible with the hardware of the day
and the text required to train the networks was not available.

These days we have a huge number of language models to choose from
for every situation from text-to-image to biological paper scanning.
I'm going to do a little experiment using the default settings of
the copilot extension to VS Code to see if we can actually write
something that works.

Note that I'm not an expert at this, not doubt there will be others
who have done little else for the last few years who can speak more
confidently about how to do this. There are also a myriad of YouTube
videos to help you on your way.

 We're using Rust for this as it is the language that I teach and
is increasingly the choice for startups and new projects and even a
few legacy ones like Linux. If you are looking to book a training course
in Rust for your existing C++ teams then this is a shameless plug.

This article is based on a workshop we held at Oxford ACCU/Rust
the first wednesday of July. Come to our sessions if you want to
learn more!



## Step 1, plan what we are going to do.

As with all coding projects, we need to plan what we are doing and decide
on the end goal we are going to reach. We are going to have to proceed in
steps and use our existing knowledge of programming to guide us on the way.

Our goal is to write a simple chess board visualisation where two players
take it in turn to move pieces on the board. For a stretch goal, we could
check the moves.

As with all Rust programs we start with `cargo new` to create a hello
world program and then edit with VS Code.

```
$ cargo new overload-vibe-coding
$ code overload-vibe-coding
```

This generates our start point:

```Rust
fn main() {
    println!("Hello, world!");
}
```

We have pre-installed the rust-analyser and copilot extensions to VS Code
and are using the default language model, GPT 4. You have a choice of several
language models, so have a play with them all.

We can now use `ctrl-I` to open the Copilot prompt.

I asked it:

```
> Write a plan for a two player chess program as a module comment
```

I'm told that creating a plan first is a good first step as it give you a chance to review
what you are about to do.

Of course, we could just write

```
> Write me a two player chess program
```

And I did try this, but the results were quite disappointing and no doubt just cribbed
from something on the internet.

The plan it generated looks like this:

```Rust
//! # Two-Player Chess Program
//!
//! This module provides the foundation for implementing a two-player chess game in Rust.
//! The program will include the following key components:
//!
//! ## Features
//! - **Game Board Representation**: A data structure to represent an 8x8 chessboard, including pieces and their positions.
//! - **Piece Movement Rules**: Logic to validate legal moves for each type of chess piece (pawn, knight, bishop, rook, queen, king).
//! - **Turn-Based Gameplay**: Alternating turns between two players, enforcing the rules of chess.
//! - **Check and Checkmate Detection**: Mechanisms to detect when a king is in check or checkmate.
//! - **Stalemate and Draw Conditions**: Logic to handle stalemates, insufficient material, and other draw scenarios.
//! - **Move History**: A record of all moves made during the game for undo functionality or analysis.
//!
//! ## Plan
//! 1. **Define Data Structures**:
//!    - Create enums for `PieceType` (pawn, knight, etc.) and `Color` (white, black).
//!    - Define a `Piece` struct to represent a chess piece with its type and color.
//!    - Implement a `Board` struct to represent the chessboard as an 8x8 grid.
//!
//! 2. **Implement Movement Logic**:
//!    - Write functions to validate moves for each piece type based on chess rules.
//!    - Ensure moves respect the board boundaries and piece interactions (e.g., capturing, blocking).
//! 
//! etc.
```

This sounds credible, so let's proceed. We should be cautious of the origins of such text as it may
be subject to copyright however.

## Step 2, A chess piece and board square.

Let's define an `enum` for a chess square. We will include a blank square so that we don't need to
use and `Option` for blank squares.

The query `> define an enum for chess pieces with colour and a blank square` was disappointing 
and the answer was quite verbose.

A few more queries produced a very similar result and I had to repeat the query a few times until
I had a similar result to what I expected. This enum should occupy two bytes, one for the
discriminator and one for the colour.

```Rust
// > make an enum ChessPiece
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChessPiece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}
```

I would imagine that the source weights had a few variants that it was cycling through.

I tried to add an extra element `Blank` to the enum, but manually editing worked better.
I also corrected the spelling to European English.

Next

```Rust
// > make a struct GameState with an 8x8 board and a player colour.
#[derive(Debug)]
struct GameState {
    board: [[ChessPiece; 8]; 8],
    current_player: Colour,
}
```

From now on I'll show just the final result of each query. I would imagine
that the queries would be better with a more experienced "vibe" coder.

## Step 3, The game state.

The local `use` items here is not in common usage in Rust, but I find it reduces
the source code a great deal so I edited the result. Very few Rust examples
use local `use` to reduce code bloat like this.

```Rust
// > add a constructor to GameState using an 8x8 const array
fn new() -> Self {
    use ChessPiece::*; // These were manual edits to make the code shorter.
    use Colour::*;
    const INITIAL_BOARD: [[ChessPiece; 8]; 8] = [
        [Rook(White), Knight(White), Bishop(White), Queen(White),
            King(White), Bishop(White), Knight(White), Rook(White)],
        [Pawn(White); 8],
        [Blank; 8],
        [Blank; 8],
        [Blank; 8],
        [Blank; 8],
        [Pawn(Black); 8],
        [Rook(Black), Knight(Black), Bishop(Black), Queen(Black),
            King(Black), Bishop(Black), Knight(Black), Rook(Black)],
    ];

    GameState {
        board: INITIAL_BOARD,
        current_player: Colour::White,
    }
}
```

A general observation is that I'm being steered in the direction of the original
coder of this problem and making the code *yours* would be more challenging
for a less experienced coder.

But on the whole, it has made something that seems to work with very few edits.

```Rust
// > implement a method to convert Square to a row and column
impl Square {
    fn to_row_col(self) -> (usize, usize) {
        let index = self as usize;
        let row = index / 8;
        let col = index % 8;
        (row, col)
    }
}
```

And now let's show the board and next player:

```Rust
// > implement Display for GameState with labeled rows and columns
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  A B C D E F G H")?;
        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{} ", 8 - i)?; // Row labels (8 to 1)
            for piece in row.iter() {
                let symbol = match piece {
                    ChessPiece::Pawn(Colour::White) => "♙",
                    ChessPiece::Knight(Colour::White) => "♘",
                    // ...
                    ChessPiece::King(Colour::Black) => "♚",
                    ChessPiece::Blank => ".",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  A B C D E F G H")?;
        writeln!(f, "Current Player: {:?}", self.current_player)
    }
}
```

Let's convert strings into squares. This is a bit of a hacky solution
as we could have parsed the chars and `to_uppercase()` allocates memory
which is a no-no in system languages.

```Rust
// > implement FromStr for square
impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A1" => Ok(Square::A1), "A2" => Ok(Square::A2), "A3" => Ok(Square::A3), "A4" => Ok(Square::A4),
            // ...
            "H5" => Ok(Square::H5), "H6" => Ok(Square::H6), "H7" => Ok(Square::H7), "H8" => Ok(Square::H8),
            _ => Err(format!("Invalid square: {}", s)),
        }
    }
}
```

Test generation did a lot more work than I asked and made a number of tests for `Square`.

```Rust
// > test that to_row_col returns (0,0) for A1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_row_col_a1() {
        let square = Square::A1;
        let (row, col) = square.to_row_col();
        assert_eq!((row, col), (0, 0));
    }

    // ...
}
```

Finally we add a game loop:

```Rust
// > add a loop to display the board and accept moves
fn main() {
    let mut game_state = GameState::new();

    loop {
        println!("{}", game_state);

        println!("Enter your move (e.g., 'e2 e4') or 'quit' to exit:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        // ...
        if let Err(err) = game_state.make_move(from_square, to_square) {
            println!("Invalid move: {}", err);
            continue;
        }
    }
}
   
// > implement make_move
fn make_move(&mut self, from: Square, to: Square) -> Result<(), String> {
    let (from_row, from_col) = from.to_row_col();
    let (to_row, to_col) = to.to_row_col();
    // ...
}

```

So now we are ready to run a system test:

```
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/overload-vibe-coding`
  A B C D E F G H
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
6 . . . . . . . . 
5 . . . . . . . . 
4 . . . . . . . . 
3 . . . . . . . . 
2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
  A B C D E F G H
Current Player: White

Enter your move (e.g., 'e2 e4') or 'quit' to exit:
E2 E4
Invalid move: No piece at the source square.
```

Clearly something is wrong.

* What seem to be white pieces are actually black.
* The board is drawn upside down.
* from_row_col swaps rows for columns.

Interestingly, these are all mistakes that humans would
make. The code "looked" good but was rotten at its core.

Fixing these problems (manually, because for me it is easier)
gives a better result where the moves actually work correctly.

```
...
Enter your move (e.g., 'e2 e4') or 'quit' to exit:
e2 e4
  A B C D E F G H
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
6 . . . . . . . . 
5 . . . . . . . . 
4 . . . . ♟ . . . 
3 . . . . . . . . 
2 ♟ ♟ ♟ ♟ . ♟ ♟ ♟ 
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
  A B C D E F G H
Current Player: Black
```

Clearly we can continue to develop the plan, but we will run out of
paper.

## Conclusions

Yes. It is possible to write code with LLMs but it does
need some looking after. It will try to push you towards the
program it was trained on. No doubt this will improve
with more training as more examples become available.

The quality of code generated by LLMs is highly dependent
on the quality of the code of the training set.
For example, the use of to_uppercase() shown above isn't ideal.
A junior coder would not know why this is a bad idea having not
spent hours on Godbolt checking the codegen.

The LLM *is* very useful for looking things up, like
ANSI escape codes or unicode characters. It is also useful
for completion, although rust-analyser's completion
is very often more targeted.

LLMs are very useful for teaching you a new programming language.
For learners of Rust, they must be a great help. I've learned
most of my programming languages from reference manuals,
and I would not wish that on a new generation, especially
one with hundreds of programming languages to learn.

For tests, LLMs take a lot of the pain out of the process
and seem to be able to get good coverage. You should review
the results, however.

On the whole, I think that LLMs are here to stay. For
CRUD farming and front-end development it will become
a necessary tool and even more eclectic things such
as compiler development can benefit from the completion
and lookup LLMs offer. We do need to take care and review
the generated code, however, as what looks convincing
may not actually work.
