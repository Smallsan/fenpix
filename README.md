# Fenpix Library

Fenpix is a Rust library that converts FEN strings into pixel chess boards.

The pixel assets were designed by me, so you're free to use them as you like.

## Features

### fen_to_board_image

Converts fen string ex. "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR" into a pixel art chess board.

![Pixel Board](https://github.com/Smallsan/fenpix/raw/master/chess_board.png)

### fen_to_board_buffer

Converts fen string to a chess board image buffer.

## Usage

Using it is pretty simple.

```rust
// import the library.
use fenix::*;

// The function fen_to_board_img takes 3 parameters.
// The fen string, The image output directory and The upscale multiplier.
fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "chess_board.png", 1, ChessAssets::default());

// The function fen_to_board_buffer takes 2 parameters.
// The fen string and the upscale multiplier.
let img_buffer = fen_to_board_buffer("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", 1, ChessAssets::default());

// The function above is inefficient because it loads all the chess assets everytime the function is called.
// If you want something more peformant you should load the assets and store them in a variable.

fn main() {
    let assets = ChessAssets::default();

    loop {
        fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "chess_board.png", 1, assets);

        let img_buffer = fen_to_board_buffer("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", 1, assets);
    }
}

```
