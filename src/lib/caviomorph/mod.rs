use std::ptr::null;
use bincode::{config, Decode, Encode};

use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use crate::helpers::modulo;


#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct BundledLog{
    pub keys: Vec<char>
}


#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct EncodedImageDelta{
    pub width: u16,
    pub height: u16,
    pub updates: Vec<PixelUpdate>,
    pub blankout: bool, // Set to true if all blocks should reset to blankout color
    pub blankout_color: u8 // Color to set all blocks to if blankout is true

}
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct PixelUpdate {
    pub row: u16,
    pub left_offset: u16,
    pub pixels: Vec<PixelBlock>
}
#[derive(Encode, Decode, PartialEq, Debug, Copy, Clone)]
pub struct PixelBlock{
    pub color: i32,
    pub count: u16

}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
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

}

fn get_subblock(image: &RawImage, x: u16, y: u16, width: u16, height: u16) -> ImageBlock{
    let mut block = ImageBlock{
        width: width,
        height: height,
        pixels: Vec::new()
    };
    println!("Getting subblock: {}, {}, {}, {}", x, y, width, height);
    println!("Pixels: {}", image.pixels.len());
    for i in 0..height{
        for j in 0..width{
            block.pixels.push(image.pixels[((y + j) + (x + i)*image.width)as usize]); //Todo Fix
        }
    }
    return block;
}

fn subblock_diff(image: RawImage, i: u16, other: RawImage, tx: Sender<PixelUpdate>){
    //TODO implment
    let block_width = image.width / 4;
    let block_height = image.height / 4;
    println!("Block width: {}, Block height: {}", block_width, block_height);
    let original_block = get_subblock(&image, i /4 * block_width, modulo(i,4) * block_height, block_width, block_height);
    let new_block = get_subblock(&other, i /4 * block_width, modulo(i,4) * block_height, block_width, block_height);
    // Get the differences in the pixels for each block
    let mut change_has_occurred = false;
    for j in 0..original_block.pixels.len() {
        if original_block.pixels[j] != new_block.pixels[j]{
            change_has_occurred = true;
            let mut len = 0;
            let mut run = true;
            while (run){
                if new_block.pixels[j] == new_block.pixels[j + len]{
                    len +=  1;
                }else{
                    run = false;
                }
            }
            let update = PixelUpdate{
                row: i as u16 / 4 * block_height + j as u16 / block_width,
                left_offset: modulo(i, 4) * block_width + j as u16 % block_width,
                pixels: vec![PixelBlock{
                    color: new_block.pixels[j] as i32,
                    count: len as u16
                }]
            };
            tx.send(update).unwrap();

        }
        //changelog.push(original_block.pixels[j] != new_block.pixels[j]);
    }

    if change_has_occurred{
        println!("Change has occurred");
    }else{
        println!("No change has occurred");
    }
}
pub fn diff(image: &RawImage, other: &RawImage) -> EncodedImageDelta{
    let mut delta = EncodedImageDelta{
        width: image.width,
        height: image.height,
        updates: Vec::new(),
        blankout: false,
        blankout_color: 0
    };
    if image.width != other.width || image.height != other.height{
        delta.blankout = true;
        //TODO implement
        return delta;
    }

    // Separate the image into 16 equal sized blocks

    let mut changelog: Vec<bool> = Vec::new();
    let mut changes: u16 = 0b0000_0000_0000_0000;

    let(tx, rx) = std::sync::mpsc::channel();
    let mut threads = Vec::new();

    for i in 0..15 {
        let txa = tx.clone();
        let img = image.clone();
        let oth = other.clone();
        threads.push(std::thread::spawn( move || {
            subblock_diff(img, i as u16, oth, txa);
        }));
    }

    for thread in threads{
        thread.join().unwrap();
    }

    return delta;


}