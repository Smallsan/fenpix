# Fenpix Library

Fenpix is a Rust library that converts FEN strings into pixel chess boards.

The pixel assets were designed by me, so you're free to use them as you like.

## Features

### fen_to_board_image

converts fen string ex. "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR" into a pixel art chess board.

![Pixel Board](https://github.com/Smallsan/fenpix/raw/master/chess_board.png)

## Usage

Using it is pretty simple.

```rust
// import the fen_to_board_img function from the crate.
use fenix::fen_to_board_img;

// The function fen_to_board_img takes 3 parameters.
// The fen string, The image output directory and The upscale multiplier.
fen_to_board_img("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "chess_board.png", 1);

```
