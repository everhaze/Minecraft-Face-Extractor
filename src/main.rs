use std::process::exit;
use clap::Parser;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "in")]
    input: String,
    #[arg(short = 'o', long = "out")]
    output: String,
    #[arg(short = 'l', long = "overlay")]
    layer: String,
}

fn main() {
    let args = Args::parse();
    
    let inpath = args.input;
    let outpath = args.output;

    let i2 = RgbaImage::new(16, 16);

    let layer = args.layer;

    match layer.trim() {
        "no" => {no_overlay(inpath, outpath, i2);}
        "yes" => {overlay(inpath, outpath, i2);}
        _ => panic!("Use -l yes or -l no.")
    }
}

fn overlay(inpath: String, outpath: String, mut i2: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let i1 = image::open(inpath).unwrap_or_else(|_| {
        println!("Invalid path. Is it a png?");
        exit(1)
    });

    //stage 1
    for x in 0..=7 {
        let s1pix = i1.get_pixel(8 + x, 15);
        let o1pix = i1.get_pixel(40 + x, 15);
        i2.put_pixel(0 + x, 15, s1pix);
        check_alpha(o1pix, &mut i2, 0 + x, 15);
    }

    //stage 2
    for x in 0..=7 {
        let s2pix = i1.get_pixel(8 + x, 14);
        let o2pix = i1.get_pixel(40 + x, 14);
        i2.put_pixel(0 + x, 14, s2pix);
        check_alpha(o2pix, &mut i2, 0 + x, 14);
    }

    //stage 3
    for x in 0..=7 {
        let s3pix = i1.get_pixel(8 + x, 13);
        let o3pix = i1.get_pixel(40 + x, 13);
        i2.put_pixel(0 + x, 13, s3pix);
        check_alpha(o3pix, &mut i2, 0 + x, 13);
    }

    //stage 4
    for x in 0..=7 {
        let s4pix = i1.get_pixel(8 + x, 12);
        let o4pix = i1.get_pixel(40 + x, 12);
        i2.put_pixel(0 + x, 12, s4pix);
        check_alpha(o4pix, &mut i2, 0 + x, 12);
    }

    //stage 5
    for x in 0..=7 {
        let s5pix = i1.get_pixel(8 + x, 11);
        let o5pix = i1.get_pixel(40 + x, 11);
        i2.put_pixel(0 + x, 11, s5pix);
        check_alpha(o5pix, &mut i2, 0 + x, 11);
    }

    //stage 6
    for x in 0..=7 {
        let s6pix = i1.get_pixel(8 + x, 10);
        let o6pix = i1.get_pixel(40 + x, 10);
        i2.put_pixel(0 + x, 10, s6pix);
        check_alpha(o6pix, &mut i2, 0 + x, 10);
    }

    //stage 7
    for x in 0..=7 {
        let s7pix = i1.get_pixel(8 + x, 9);
        let o7pix = i1.get_pixel(40 + x, 9);
        i2.put_pixel(0 + x, 9, s7pix);
        check_alpha(o7pix, &mut i2, 0 + x, 9);
    }

    //stage 8
    for x in 0..=7 {
        let s8pix = i1.get_pixel(8 + x, 8);
        let o8pix = i1.get_pixel(40 + x, 8);
        i2.put_pixel(0 + x, 8, s8pix);
        check_alpha(o8pix, &mut i2, 0 + x, 8);
    }

    //save output
    i2.save(outpath).unwrap();
    println!("Success!");
}

fn no_overlay(inpath: String, outpath: String, mut i2: ImageBuffer<Rgba<u8>, Vec<u8>>) {

    let i1 = image::open(inpath).unwrap_or_else(|_| {
        println!("Invalid path. Is it a png?");
        exit(1)
    });

    //stage 1
    for x in 0..=7 {
        let s1pix = i1.get_pixel(8 + x, 15);
        i2.put_pixel(0 + x, 15, s1pix);
    }

    //stage 2
    for x in 0..=7 {
        let s2pix = i1.get_pixel(8 + x, 14);
        i2.put_pixel(0 + x, 14, s2pix);
    }

    //stage 3
    for x in 0..=7 {
        let s3pix = i1.get_pixel(8 + x, 13);
        i2.put_pixel(0 + x, 13, s3pix);
    }

    //stage 4
    for x in 0..=7 {
        let s4pix = i1.get_pixel(8 + x, 12);
        i2.put_pixel(0 + x, 12, s4pix);
    }

    //stage 5
    for x in 0..=7 {
        let s5pix = i1.get_pixel(8 + x, 11);
        i2.put_pixel(0 + x, 11, s5pix);
    }

    //stage 6
    for x in 0..=7 {
        let s6pix = i1.get_pixel(8 + x, 10);
        i2.put_pixel(0 + x, 10, s6pix);
    }

    //stage 7
    for x in 0..=7 {
        let s7pix = i1.get_pixel(8 + x, 9);
        i2.put_pixel(0 + x, 9, s7pix);
    }

    //stage 8
    for x in 0..=7 {
        let s8pix = i1.get_pixel(8 + x, 8);
        i2.put_pixel(0 + x, 8, s8pix);
    }

    //save output
    i2.save(outpath).unwrap();
    println!("Success!");
}

fn check_alpha(pixel: Rgba<u8>, i2: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32) {
    if pixel[3] != 0 {
        i2.put_pixel(x, y, pixel);
    }
}
