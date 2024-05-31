use std::collections::HashMap;

use image::{ImageFormat, RgbaImage};

use crate::errors::FenToImgError;

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

impl ChessAssets {
    pub fn new(piece_images: HashMap<char, RgbaImage>, board_image: RgbaImage) -> Self {
        ChessAssets {
            piece_images,
            board_image,
        }
    }
    // Black Pieces
    /// Piece size requirements: 16x16 Canvas size, Minimum of 1 pixel of border
    /// image_path: Path to the image file
    pub fn set_black_king(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('k', image);
        Ok(())
    }
    pub fn set_black_queen(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('q', image);
        Ok(())
    }
    pub fn set_black_rook(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('r', image);
        Ok(())
    }
    pub fn set_black_bishop(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('b', image);
        Ok(())
    }
    pub fn set_black_knight(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('n', image);
        Ok(())
    }
    pub fn set_black_pawn(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('p', image);
        Ok(())
    }
    // White Pieces
    pub fn set_white_king(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('K', image);
        Ok(())
    }
    pub fn set_white_queen(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('Q', image);
        Ok(())
    }
    pub fn set_white_rook(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('R', image);
        Ok(())
    }
    pub fn set_white_bishop(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('B', image);
        Ok(())
    }
    pub fn set_white_knight(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('N', image);
        Ok(())
    }
    pub fn set_white_pawn(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.piece_images.insert('P', image);
        Ok(())
    }
    // Board 510x510 with 9 pixels as border
    pub fn set_board(&mut self, image_path: &str) -> Result<(), FenToImgError> {
        let image = image::open(image_path).map_err(|err| FenToImgError::ImageError(err))?.to_rgba8();
        self.board_image = image;
        Ok(())
    }
    

}
