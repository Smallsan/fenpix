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
// Import the library.
use fenix::*;

// The function `fen_to_board_img` takes four parameters:
// 1. The FEN string
// 2. The image output directory
// 3. The upscale multiplier
// 4. The ChessAssets instance
// This returns an result(), or an error if it fails to convert the fen to image.
// The board also auto rotates depending on whose turn it is.
let result = fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w", "chess_board.png", 1, &ChessAssets::default());

result.unwrap();

// The function `fen_to_board_buffer` also takes three parameters:
// 1. The FEN string
// 2. The upscale multiplier
// 3. The ChessAssets instance
// This returns an result type that needs to be matched or unwrapped.
// The board also auto rotates depending on whose turn it is.
let img_buffer = fen_to_board_buffer("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b", 1, ChessAssets::default()).unwrap();

// The above functions are inefficient because they load all the chess assets every time they are called.
// For better performance, load the assets once and store them in a variable.

fn main() {
    let assets = ChessAssets::default();

    loop {
        fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w", "chess_board.png", 1, &assets).unwrap();

        let img_buffer = fen_to_board_buffer("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b", 1, &assets).unwrap();
    }
}

```
