use std::{io, process::exit};
use clap::Parser;
use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "in")]
    input: String,
    #[arg(short = 'o', long = "out")]
    output: String,
}

fn main() {
    let args = Args::parse();
    
    let inpath = args.input;
    let outpath = args.output;

    let i2 = RgbaImage::new(16, 16);

    println!("No Overlay (1)\nOverlay (2)");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    match answer.trim() {
        "1" => {no_overlay(inpath, outpath, i2);}
        "2" => {overlay(inpath, outpath, i2);}
        _ => panic!("Enter 1 or 2.")
    }
}

fn overlay(inpath: String, outpath: String, mut i2: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let i1 = image::open(inpath).unwrap_or_else(|_| {
        println!("Invalid path. Is it a png?");
        exit(1)
    });
    
    //stage 1
    let s1pix1 = i1.get_pixel(8, 15);
    let s1pix2 = i1.get_pixel(9, 15);
    let s1pix3 = i1.get_pixel(10, 15);
    let s1pix4 = i1.get_pixel(11, 15);
    let s1pix5 = i1.get_pixel(12, 15);
    let s1pix6 = i1.get_pixel(13, 15);
    let s1pix7 = i1.get_pixel(14, 15);
    let s1pix8 = i1.get_pixel(15, 15);

    let o1pix1 = i1.get_pixel(40, 15);
    let o1pix2 = i1.get_pixel(41, 15);
    let o1pix3 = i1.get_pixel(42, 15);
    let o1pix4 = i1.get_pixel(43, 15);
    let o1pix5 = i1.get_pixel(44, 15);
    let o1pix6 = i1.get_pixel(45, 15);
    let o1pix7 = i1.get_pixel(46, 15);
    let o1pix8 = i1.get_pixel(47, 15);

    //stage 2
    let s2pix1 = i1.get_pixel(8, 14);
    let s2pix2 = i1.get_pixel(9, 14);
    let s2pix3 = i1.get_pixel(10, 14);
    let s2pix4 = i1.get_pixel(11, 14);
    let s2pix5 = i1.get_pixel(12, 14);
    let s2pix6 = i1.get_pixel(13, 14);
    let s2pix7 = i1.get_pixel(14, 14);
    let s2pix8 = i1.get_pixel(15, 14);

    let o2pix1 = i1.get_pixel(40, 14);
    let o2pix2 = i1.get_pixel(41, 14);
    let o2pix3 = i1.get_pixel(42, 14);
    let o2pix4 = i1.get_pixel(43, 14);
    let o2pix5 = i1.get_pixel(44, 14);
    let o2pix6 = i1.get_pixel(45, 14);
    let o2pix7 = i1.get_pixel(46, 14);
    let o2pix8 = i1.get_pixel(47, 14);

    //stage 3
    let s3pix1 = i1.get_pixel(8, 13);
    let s3pix2 = i1.get_pixel(9, 13);
    let s3pix3 = i1.get_pixel(10, 13);
    let s3pix4 = i1.get_pixel(11, 13);
    let s3pix5 = i1.get_pixel(12, 13);
    let s3pix6 = i1.get_pixel(13, 13);
    let s3pix7 = i1.get_pixel(14, 13);
    let s3pix8 = i1.get_pixel(15, 13);

    let o3pix1 = i1.get_pixel(40, 13);
    let o3pix2 = i1.get_pixel(41, 13);
    let o3pix3 = i1.get_pixel(42, 13);
    let o3pix4 = i1.get_pixel(43, 13);
    let o3pix5 = i1.get_pixel(44, 13);
    let o3pix6 = i1.get_pixel(45, 13);
    let o3pix7 = i1.get_pixel(46, 13);
    let o3pix8 = i1.get_pixel(47, 13);

    //stage 4
    let s4pix1 = i1.get_pixel(8, 12);
    let s4pix2 = i1.get_pixel(9, 12);
    let s4pix3 = i1.get_pixel(10, 12);
    let s4pix4 = i1.get_pixel(11, 12);
    let s4pix5 = i1.get_pixel(12, 12);
    let s4pix6 = i1.get_pixel(13, 12);
    let s4pix7 = i1.get_pixel(14, 12);
    let s4pix8 = i1.get_pixel(15, 12);

    let o4pix1 = i1.get_pixel(40, 12);
    let o4pix2 = i1.get_pixel(41, 12);
    let o4pix3 = i1.get_pixel(42, 12);
    let o4pix4 = i1.get_pixel(43, 12);
    let o4pix5 = i1.get_pixel(44, 12);
    let o4pix6 = i1.get_pixel(45, 12);
    let o4pix7 = i1.get_pixel(46, 12);
    let o4pix8 = i1.get_pixel(47, 12);

    //stage 5
    let s5pix1 = i1.get_pixel(8, 11);
    let s5pix2 = i1.get_pixel(9, 11);
    let s5pix3 = i1.get_pixel(10, 11);
    let s5pix4 = i1.get_pixel(11, 11);
    let s5pix5 = i1.get_pixel(12, 11);
    let s5pix6 = i1.get_pixel(13, 11);
    let s5pix7 = i1.get_pixel(14, 11);
    let s5pix8 = i1.get_pixel(15, 11);

    let o5pix1 = i1.get_pixel(40, 11);
    let o5pix2 = i1.get_pixel(41, 11);
    let o5pix3 = i1.get_pixel(42, 11);
    let o5pix4 = i1.get_pixel(43, 11);
    let o5pix5 = i1.get_pixel(44, 11);
    let o5pix6 = i1.get_pixel(45, 11);
    let o5pix7 = i1.get_pixel(46, 11);
    let o5pix8 = i1.get_pixel(47, 11);

    //stage 6
    let s6pix1 = i1.get_pixel(8, 10);
    let s6pix2 = i1.get_pixel(9, 10);
    let s6pix3 = i1.get_pixel(10, 10);
    let s6pix4 = i1.get_pixel(11, 10);
    let s6pix5 = i1.get_pixel(12, 10);
    let s6pix6 = i1.get_pixel(13, 10);
    let s6pix7 = i1.get_pixel(14, 10);
    let s6pix8 = i1.get_pixel(15, 10);

    let o6pix1 = i1.get_pixel(40, 10);
    let o6pix2 = i1.get_pixel(41, 10);
    let o6pix3 = i1.get_pixel(42, 10);
    let o6pix4 = i1.get_pixel(43, 10);
    let o6pix5 = i1.get_pixel(44, 10);
    let o6pix6 = i1.get_pixel(45, 10);
    let o6pix7 = i1.get_pixel(46, 10);
    let o6pix8 = i1.get_pixel(47, 10);

    //stage 7
    let s7pix1 = i1.get_pixel(8, 9);
    let s7pix2 = i1.get_pixel(9, 9);
    let s7pix3 = i1.get_pixel(10, 9);
    let s7pix4 = i1.get_pixel(11, 9);
    let s7pix5 = i1.get_pixel(12, 9);
    let s7pix6 = i1.get_pixel(13, 9);
    let s7pix7 = i1.get_pixel(14, 9);
    let s7pix8 = i1.get_pixel(15, 9);

    let o7pix1 = i1.get_pixel(40, 9);
    let o7pix2 = i1.get_pixel(41, 9);
    let o7pix3 = i1.get_pixel(42, 9);
    let o7pix4 = i1.get_pixel(43, 9);
    let o7pix5 = i1.get_pixel(44, 9);
    let o7pix6 = i1.get_pixel(45, 9);
    let o7pix7 = i1.get_pixel(46, 9);
    let o7pix8 = i1.get_pixel(47, 9);

    //stage 8
    let s8pix1 = i1.get_pixel(8, 8);
    let s8pix2 = i1.get_pixel(9, 8);
    let s8pix3 = i1.get_pixel(10, 8);
    let s8pix4 = i1.get_pixel(11, 8);
    let s8pix5 = i1.get_pixel(12, 8);
    let s8pix6 = i1.get_pixel(13, 8);
    let s8pix7 = i1.get_pixel(14, 8);
    let s8pix8 = i1.get_pixel(15, 8);

    let o8pix1 = i1.get_pixel(40, 8);
    let o8pix2 = i1.get_pixel(41, 8);
    let o8pix3 = i1.get_pixel(42, 8);
    let o8pix4 = i1.get_pixel(43, 8);
    let o8pix5 = i1.get_pixel(44, 8);
    let o8pix6 = i1.get_pixel(45, 8);
    let o8pix7 = i1.get_pixel(46, 8);
    let o8pix8 = i1.get_pixel(47, 8);

    //build pixel by pixel

    //stage 1
    i2.put_pixel(0, 15, s1pix1);
    i2.put_pixel(1, 15, s1pix2);
    i2.put_pixel(2, 15, s1pix3);
    i2.put_pixel(3, 15, s1pix4);
    i2.put_pixel(4, 15, s1pix5);
    i2.put_pixel(5, 15, s1pix6);
    i2.put_pixel(6, 15, s1pix7);
    i2.put_pixel(7, 15, s1pix8);

    check_alpha(o1pix1, &mut i2, 0, 15);
    check_alpha(o1pix2, &mut i2, 1, 15);
    check_alpha(o1pix3, &mut i2, 2, 15);
    check_alpha(o1pix4, &mut i2, 3, 15);
    check_alpha(o1pix5, &mut i2, 4, 15);
    check_alpha(o1pix6, &mut i2, 5, 15);
    check_alpha(o1pix7, &mut i2, 6, 15);
    check_alpha(o1pix8, &mut i2, 7, 15);
    
    //stage 2
    i2.put_pixel(0, 14, s2pix1);
    i2.put_pixel(1, 14, s2pix2);
    i2.put_pixel(2, 14, s2pix3);
    i2.put_pixel(3, 14, s2pix4);
    i2.put_pixel(4, 14, s2pix5);
    i2.put_pixel(5, 14, s2pix6);
    i2.put_pixel(6, 14, s2pix7);
    i2.put_pixel(7, 14, s2pix8);

    check_alpha(o2pix1, &mut i2, 0, 14);
    check_alpha(o2pix2, &mut i2, 1, 14);
    check_alpha(o2pix3, &mut i2, 2, 14);
    check_alpha(o2pix4, &mut i2, 3, 14);
    check_alpha(o2pix5, &mut i2, 4, 14);
    check_alpha(o2pix6, &mut i2, 5, 14);
    check_alpha(o2pix7, &mut i2, 6, 14);
    check_alpha(o2pix8, &mut i2, 7, 14);

    //stage 3
    i2.put_pixel(0, 13, s3pix1);
    i2.put_pixel(1, 13, s3pix2);
    i2.put_pixel(2, 13, s3pix3);
    i2.put_pixel(3, 13, s3pix4);
    i2.put_pixel(4, 13, s3pix5);
    i2.put_pixel(5, 13, s3pix6);
    i2.put_pixel(6, 13, s3pix7);
    i2.put_pixel(7, 13, s3pix8);

    check_alpha(o3pix1, &mut i2, 0, 13);
    check_alpha(o3pix2, &mut i2, 1, 13);
    check_alpha(o3pix3, &mut i2, 2, 13);
    check_alpha(o3pix4, &mut i2, 3, 13);
    check_alpha(o3pix5, &mut i2, 4, 13);
    check_alpha(o3pix6, &mut i2, 5, 13);
    check_alpha(o3pix7, &mut i2, 6, 13);
    check_alpha(o3pix8, &mut i2, 7, 13);

    //stage 4
    i2.put_pixel(0, 12, s4pix1);
    i2.put_pixel(1, 12, s4pix2);
    i2.put_pixel(2, 12, s4pix3);
    i2.put_pixel(3, 12, s4pix4);
    i2.put_pixel(4, 12, s4pix5);
    i2.put_pixel(5, 12, s4pix6);
    i2.put_pixel(6, 12, s4pix7);
    i2.put_pixel(7, 12, s4pix8);

    check_alpha(o4pix1, &mut i2, 0, 12);
    check_alpha(o4pix2, &mut i2, 1, 12);
    check_alpha(o4pix3, &mut i2, 2, 12);
    check_alpha(o4pix4, &mut i2, 3, 12);
    check_alpha(o4pix5, &mut i2, 4, 12);
    check_alpha(o4pix6, &mut i2, 5, 12);
    check_alpha(o4pix7, &mut i2, 6, 12);
    check_alpha(o4pix8, &mut i2, 7, 12);

    //stage 5
    i2.put_pixel(0, 11, s5pix1);
    i2.put_pixel(1, 11, s5pix2);
    i2.put_pixel(2, 11, s5pix3);
    i2.put_pixel(3, 11, s5pix4);
    i2.put_pixel(4, 11, s5pix5);
    i2.put_pixel(5, 11, s5pix6);
    i2.put_pixel(6, 11, s5pix7);
    i2.put_pixel(7, 11, s5pix8);

    check_alpha(o5pix1, &mut i2, 0, 11);
    check_alpha(o5pix2, &mut i2, 1, 11);
    check_alpha(o5pix3, &mut i2, 2, 11);
    check_alpha(o5pix4, &mut i2, 3, 11);
    check_alpha(o5pix5, &mut i2, 4, 11);
    check_alpha(o5pix6, &mut i2, 5, 11);
    check_alpha(o5pix7, &mut i2, 6, 11);
    check_alpha(o5pix8, &mut i2, 7, 11);

    //stage 6
    i2.put_pixel(0, 10, s6pix1);
    i2.put_pixel(1, 10, s6pix2);
    i2.put_pixel(2, 10, s6pix3);
    i2.put_pixel(3, 10, s6pix4);
    i2.put_pixel(4, 10, s6pix5);
    i2.put_pixel(5, 10, s6pix6);
    i2.put_pixel(6, 10, s6pix7);
    i2.put_pixel(7, 10, s6pix8);

    check_alpha(o6pix1, &mut i2, 0, 10);
    check_alpha(o6pix2, &mut i2, 1, 10);
    check_alpha(o6pix3, &mut i2, 2, 10);
    check_alpha(o6pix4, &mut i2, 3, 10);
    check_alpha(o6pix5, &mut i2, 4, 10);
    check_alpha(o6pix6, &mut i2, 5, 10);
    check_alpha(o6pix7, &mut i2, 6, 10);
    check_alpha(o6pix8, &mut i2, 7, 10);

    //stage 7
    i2.put_pixel(0, 9, s7pix1);
    i2.put_pixel(1, 9, s7pix2);
    i2.put_pixel(2, 9, s7pix3);
    i2.put_pixel(3, 9, s7pix4);
    i2.put_pixel(4, 9, s7pix5);
    i2.put_pixel(5, 9, s7pix6);
    i2.put_pixel(6, 9, s7pix7);
    i2.put_pixel(7, 9, s7pix8);

    check_alpha(o7pix1, &mut i2, 0, 9);
    check_alpha(o7pix2, &mut i2, 1, 9);
    check_alpha(o7pix3, &mut i2, 2, 9);
    check_alpha(o7pix4, &mut i2, 3, 9);
    check_alpha(o7pix5, &mut i2, 4, 9);
    check_alpha(o7pix6, &mut i2, 5, 9);
    check_alpha(o7pix7, &mut i2, 6, 9);
    check_alpha(o7pix8, &mut i2, 7, 9);

    //stage 8
    i2.put_pixel(0,8, s8pix1);
    i2.put_pixel(1, 8, s8pix2);
    i2.put_pixel(2, 8, s8pix3);
    i2.put_pixel(3, 8, s8pix4);
    i2.put_pixel(4, 8, s8pix5);
    i2.put_pixel(5, 8, s8pix6);
    i2.put_pixel(6, 8, s8pix7);
    i2.put_pixel(7, 8, s8pix8);

    check_alpha(o8pix1, &mut i2, 0, 8);
    check_alpha(o8pix2, &mut i2, 1, 8);
    check_alpha(o8pix3, &mut i2, 2, 8);
    check_alpha(o8pix4, &mut i2, 3, 8);
    check_alpha(o8pix5, &mut i2, 4, 8);
    check_alpha(o8pix6, &mut i2, 5, 8);
    check_alpha(o8pix7, &mut i2, 6, 8);
    check_alpha(o8pix8, &mut i2, 7, 8);

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
    let s1pix1 = i1.get_pixel(8, 15);
    let s1pix2 = i1.get_pixel(9, 15);
    let s1pix3 = i1.get_pixel(10, 15);
    let s1pix4 = i1.get_pixel(11, 15);
    let s1pix5 = i1.get_pixel(12, 15);
    let s1pix6 = i1.get_pixel(13, 15);
    let s1pix7 = i1.get_pixel(14, 15);
    let s1pix8 = i1.get_pixel(15, 15);

    //stage 2
    let s2pix1 = i1.get_pixel(8, 14);
    let s2pix2 = i1.get_pixel(9, 14);
    let s2pix3 = i1.get_pixel(10, 14);
    let s2pix4 = i1.get_pixel(11, 14);
    let s2pix5 = i1.get_pixel(12, 14);
    let s2pix6 = i1.get_pixel(13, 14);
    let s2pix7 = i1.get_pixel(14, 14);
    let s2pix8 = i1.get_pixel(15, 14);

    //stage 3
    let s3pix1 = i1.get_pixel(8, 13);
    let s3pix2 = i1.get_pixel(9, 13);
    let s3pix3 = i1.get_pixel(10, 13);
    let s3pix4 = i1.get_pixel(11, 13);
    let s3pix5 = i1.get_pixel(12, 13);
    let s3pix6 = i1.get_pixel(13, 13);
    let s3pix7 = i1.get_pixel(14, 13);
    let s3pix8 = i1.get_pixel(15, 13);

    //stage 4
    let s4pix1 = i1.get_pixel(8, 12);
    let s4pix2 = i1.get_pixel(9, 12);
    let s4pix3 = i1.get_pixel(10, 12);
    let s4pix4 = i1.get_pixel(11, 12);
    let s4pix5 = i1.get_pixel(12, 12);
    let s4pix6 = i1.get_pixel(13, 12);
    let s4pix7 = i1.get_pixel(14, 12);
    let s4pix8 = i1.get_pixel(15, 12);

    //stage 5
    let s5pix1 = i1.get_pixel(8, 11);
    let s5pix2 = i1.get_pixel(9, 11);
    let s5pix3 = i1.get_pixel(10, 11);
    let s5pix4 = i1.get_pixel(11, 11);
    let s5pix5 = i1.get_pixel(12, 11);
    let s5pix6 = i1.get_pixel(13, 11);
    let s5pix7 = i1.get_pixel(14, 11);
    let s5pix8 = i1.get_pixel(15, 11);

    //stage 6
    let s6pix1 = i1.get_pixel(8, 10);
    let s6pix2 = i1.get_pixel(9, 10);
    let s6pix3 = i1.get_pixel(10, 10);
    let s6pix4 = i1.get_pixel(11, 10);
    let s6pix5 = i1.get_pixel(12, 10);
    let s6pix6 = i1.get_pixel(13, 10);
    let s6pix7 = i1.get_pixel(14, 10);
    let s6pix8 = i1.get_pixel(15, 10);

    //stage 7
    let s7pix1 = i1.get_pixel(8, 9);
    let s7pix2 = i1.get_pixel(9, 9);
    let s7pix3 = i1.get_pixel(10, 9);
    let s7pix4 = i1.get_pixel(11, 9);
    let s7pix5 = i1.get_pixel(12, 9);
    let s7pix6 = i1.get_pixel(13, 9);
    let s7pix7 = i1.get_pixel(14, 9);
    let s7pix8 = i1.get_pixel(15, 9);

    //stage 8
    let s8pix1 = i1.get_pixel(8, 8);
    let s8pix2 = i1.get_pixel(9, 8);
    let s8pix3 = i1.get_pixel(10, 8);
    let s8pix4 = i1.get_pixel(11, 8);
    let s8pix5 = i1.get_pixel(12, 8);
    let s8pix6 = i1.get_pixel(13, 8);
    let s8pix7 = i1.get_pixel(14, 8);
    let s8pix8 = i1.get_pixel(15, 8);

    //build pixel by pixel

    //stage 1
    i2.put_pixel(0, 15, s1pix1);
    i2.put_pixel(1, 15, s1pix2);
    i2.put_pixel(2, 15, s1pix3);
    i2.put_pixel(3, 15, s1pix4);
    i2.put_pixel(4, 15, s1pix5);
    i2.put_pixel(5, 15, s1pix6);
    i2.put_pixel(6, 15, s1pix7);
    i2.put_pixel(7, 15, s1pix8);
    
    //stage 2
    i2.put_pixel(0, 14, s2pix1);
    i2.put_pixel(1, 14, s2pix2);
    i2.put_pixel(2, 14, s2pix3);
    i2.put_pixel(3, 14, s2pix4);
    i2.put_pixel(4, 14, s2pix5);
    i2.put_pixel(5, 14, s2pix6);
    i2.put_pixel(6, 14, s2pix7);
    i2.put_pixel(7, 14, s2pix8);

    //stage 3
    i2.put_pixel(0, 13, s3pix1);
    i2.put_pixel(1, 13, s3pix2);
    i2.put_pixel(2, 13, s3pix3);
    i2.put_pixel(3, 13, s3pix4);
    i2.put_pixel(4, 13, s3pix5);
    i2.put_pixel(5, 13, s3pix6);
    i2.put_pixel(6, 13, s3pix7);
    i2.put_pixel(7, 13, s3pix8);

    //stage 4
    i2.put_pixel(0, 12, s4pix1);
    i2.put_pixel(1, 12, s4pix2);
    i2.put_pixel(2, 12, s4pix3);
    i2.put_pixel(3, 12, s4pix4);
    i2.put_pixel(4, 12, s4pix5);
    i2.put_pixel(5, 12, s4pix6);
    i2.put_pixel(6, 12, s4pix7);
    i2.put_pixel(7, 12, s4pix8);

    //stage 5
    i2.put_pixel(0, 11, s5pix1);
    i2.put_pixel(1, 11, s5pix2);
    i2.put_pixel(2, 11, s5pix3);
    i2.put_pixel(3, 11, s5pix4);
    i2.put_pixel(4, 11, s5pix5);
    i2.put_pixel(5, 11, s5pix6);
    i2.put_pixel(6, 11, s5pix7);
    i2.put_pixel(7, 11, s5pix8);

    //stage 6
    i2.put_pixel(0, 10, s6pix1);
    i2.put_pixel(1, 10, s6pix2);
    i2.put_pixel(2, 10, s6pix3);
    i2.put_pixel(3, 10, s6pix4);
    i2.put_pixel(4, 10, s6pix5);
    i2.put_pixel(5, 10, s6pix6);
    i2.put_pixel(6, 10, s6pix7);
    i2.put_pixel(7, 10, s6pix8);

    //stage 7
    i2.put_pixel(0, 9, s7pix1);
    i2.put_pixel(1, 9, s7pix2);
    i2.put_pixel(2, 9, s7pix3);
    i2.put_pixel(3, 9, s7pix4);
    i2.put_pixel(4, 9, s7pix5);
    i2.put_pixel(5, 9, s7pix6);
    i2.put_pixel(6, 9, s7pix7);
    i2.put_pixel(7, 9, s7pix8);

    //stage 8
    i2.put_pixel(0,8, s8pix1);
    i2.put_pixel(1, 8, s8pix2);
    i2.put_pixel(2, 8, s8pix3);
    i2.put_pixel(3, 8, s8pix4);
    i2.put_pixel(4, 8, s8pix5);
    i2.put_pixel(5, 8, s8pix6);
    i2.put_pixel(6, 8, s8pix7);
    i2.put_pixel(7, 8, s8pix8);

    //save output
    i2.save(outpath).unwrap();
    println!("Success!");
}

fn check_alpha(pixel: Rgba<u8>, i2: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32) {
    if pixel[3] != 0 {
        i2.put_pixel(x, y, pixel);
    }
}
