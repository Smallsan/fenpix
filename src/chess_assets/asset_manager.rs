use std::collections::HashMap;

use image::{ImageFormat, RgbaImage};

pub struct ChessAssets {
    pub piece_images: HashMap<char, RgbaImage>,
    pub board_image: RgbaImage,
}

impl Default for ChessAssets {
    fn default() -> Self {
        let piece_images: HashMap<char, RgbaImage> = [
            // Black Pieces
            (
                'p',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_pawn.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'r',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_rook.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'n',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_knight.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'b',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_bishop.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'q',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_queen.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'k',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/black_king.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            // White Pieces
            (
                'P',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_pawn.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'R',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_rook.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'N',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_knight.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'B',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_bishop.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'Q',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_queen.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
            (
                'K',
                image::load_from_memory_with_format(
                    include_bytes!("pieces/white_king.png"),
                    ImageFormat::Png,
                )
                .unwrap()
                .into_rgba8(),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let board_image = image::load_from_memory_with_format(
            include_bytes!("board/board.png"),
            ImageFormat::Png,
        )
        .unwrap()
        .into_rgba8();
        ChessAssets {
            piece_images,
            board_image,
        }
    }
}
