use bincode::{config, Decode, Encode};
use crate::helpers::modulo;


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BundledLog{
    pub keys: Vec<char>
}


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct EncodedImageDelta{
    pub width: u16,
    pub height: u16,
    pub updates: Vec<PixelUpdate>,
    pub blankout: bool, // Set to true if all blocks should reset to blankout color
    pub blankout_color: u8 // Color to set all blocks to if blankout is true

}
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct PixelUpdate {
    pub row: u16,
    pub left_offset: u16,
    pub pixels: Vec<PixelBlock>
}
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct PixelBlock{
    pub color: i32,
    pub count: u16

}

pub struct RawImage{
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u32>
}

pub struct ImageState{
    pub width: u16,
    pub height: u16,
    pub activeImage: RawImage
}

pub struct ImageBlock{
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u32>
}

impl RawImage{
    pub fn get_subblock(&self, x: u16, y: u16, width: u16, height: u16) -> ImageBlock{
        let mut block = ImageBlock{
            width: width,
            height: height,
            pixels: Vec::new()
        };
        for i in 0..height{
            for j in 0..width{
                block.pixels.push(self.pixels[((y + i) * self.width + (x + j)) as usize]);
            }
        }
        return block;
    }
    pub fn diff(&self, other: &RawImage) -> EncodedImageDelta{
        let mut delta = EncodedImageDelta{
            width: self.width,
            height: self.height,
            updates: Vec::new(),
            blankout: false,
            blankout_color: 0
        };
        if self.width != other.width || self.height != other.height{
            delta.blankout = true;
            //TODO implement
            return delta;
        }

        // Separate the image into 16 equal sized blocks
        let block_width = self.width / 4;
        let block_height = self.height / 4;
        let mut changelog: Vec<bool> = Vec::new();
        let mut changes: u16 = 0b0000_0000_0000_0000;


        for i in 0..15 {
            let original_block = self.get_subblock(i /4 * block_width, modulo(i,4) * block_height, block_width, block_height);
            let new_block = other.get_subblock(i /4 * block_width, modulo(i,4) * block_height, block_width, block_height);
            // Get the differences in the pixels for each block
            let mut change_has_occurred = false;
            for j in 0..original_block.pixels.len() {
                if original_block.pixels[j] != new_block.pixels[j]{
                    change_has_occurred = true;
                }
                changelog.push(original_block.pixels[j] != new_block.pixels[j]);
            }

            if change_has_occurred{
                println!("Change has occurred");
                changes |= 1 << i;
            }else{
                println!("No change has occurred");
            }
        }
        for entry in changelog{
        }

        return delta;


    }
}

