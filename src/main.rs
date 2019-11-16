use image::{GenericImageView, GenericImage, DynamicImage};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::{io, io::Write, fs::File};

fn main() {
    let mut _file1 = None;
    let mut _file2 = None;

    // TODO ADD SUPPORT FOR OUTPUT PATH
    let mut write_to_disk = false;
    let mut write_json = false;
    
    let mut output = "output.gif";

    let mut frame : f32 = 10.0;
    let mut t : f32 = 100.0 / 10.0;

    let mut resize = false;
    let mut resize_width = None;
    let mut resize_height = None;
    
    let mut conversion_speed : i32 = 10;

    let args : Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage : fade <file 1> <file 2> [options]");
        println!("Options :");
        println!("\t-o <output path> Set output path.");
        println!("\t-w Write frames to disk.");
        println!("\t-a Write a .json used by apngasm.");
        println!("\t-n <count> Set frames count.");
        println!("\t-s <speed> Set gif conversion speed. Must be between 1 and 30, 30 is loss quality but faster.");
        println!("\t-r <width> <height> Resize image.");
        std::process::exit(0);
    }
    
    for (i, arg) in args.iter().enumerate() {
        if i == 1 {
            _file1 = Some(arg.as_str());
        } else if i == 2 {
            _file2 = Some(arg.as_str());
        } else {
            match arg.as_str() {
                "-o" => {
                    if args.len() > i + 1 {
                        output = args[i + 1].as_str();
                    } else {
                        panic!("Missing output path");
                    }
                },
                "-w" => write_to_disk = true,
                "-n" => {
                    if args.len() > i + 1 {
                        frame =  args[i + 1].parse::<f32>().unwrap();
                        t = 100.0 / frame;
                    } else {
                        panic!("Missing arg for number of frames");
                    }
                },
                "-s" => {
                    if args.len() > i + 1 {
                        conversion_speed =  args[i + 1].parse::<i32>().unwrap();
                    } else {
                        panic!("Missing arg for conversion speed");
                    }
                },
                "-r" => {
                    if args.len() > i + 2 {
                        resize = true;
                        resize_width =  Some(args[i + 1].parse::<u32>().unwrap());
                        resize_height = Some(args[i + 2].parse::<u32>().unwrap())
                    } else {
                        panic!("Missing arg for conversion speed");
                    }
                },
                "-a" => write_json = true,
                _ => continue,
            }
        }
    }

    let file1 : &str = match _file1 {
        Some(x) => x,
        None => {
            eprintln!("File 1 must be set");
            std::process::exit(-1);
        },
    };

    let file2 : &str = match _file2 {
        Some(x) => x,
        None => {
            eprintln!("File 2 must be set"); 
            std::process::exit(-1);
        },
    };

    if !write_to_disk && write_json {
        eprintln!("Warning : You write duration to disk but not frames ?");
    }

    print!("Parameters :\n");
    print!("\tFile 1 : {}\n", file1);
    print!("\tFile 2 : {}\n", file2);
    print!("\tOutput : {}\n", output);
    print!("\tTotal of frames : {}, with a delay of : {}ms\n", frame, t);
    print!("\tWrite frames to disk : {}\n", write_to_disk);
    print!("\tWrite .json for apngasm : {}\n", write_json);
    print!("\tSpeed of conversion : {}\n", conversion_speed);
    if resize {
        print!("\tResize : true, width : {}, height : {}\n\n", resize_width.unwrap(), resize_height.unwrap());
    } else {
        print!("\tResize : false\n\n");
    }
    io::stdout().flush().ok().expect("Could not flush stdout");

    {
        let path1 = std::path::Path::new(file1);
        if !path1.exists() {
            eprintln!("Error : File 1 ({}) doesn't exist !", file1);
            std::process::exit(-1)
        } else if path1.is_dir() {
            eprintln!("Error File 1 ({}) is a directory", file1);
            std::process::exit(-1);
        }

        let path2 = std::path::Path::new(file2);
        if !path2.exists() {
            eprintln!("Error : File 2 ({}) doesn't exist !", file2);
            std::process::exit(-1)
        } else if path2.is_dir() {
            eprintln!("Error File 1 ({}) is a directory", file2);
            std::process::exit(-1);
        }
    }

    let mut img1 : DynamicImage = image::open(file1).unwrap_or_else(|error| {
        eprintln!("Error when decoding img1 : {}", error);
        std::process::exit(-1);
    });

    let mut img2 : DynamicImage = image::open(file2).unwrap_or_else(|error| {
        eprintln!("Error when decoding img2 : {}", error);
        std::process::exit(-1);
    });

    if resize {
        img1 = img1.resize(resize_width.unwrap(), resize_height.unwrap(), image::imageops::FilterType::Nearest);

        img2 = img2.resize(resize_width.unwrap(), resize_height.unwrap(), image::imageops::FilterType::Nearest);
    }

    if write_to_disk {
        let img_file1 = std::fs::File::create("0000.png").unwrap_or_else(|error| {
            eprintln!("Error when creating file : {}", error);
            std::process::exit(-1);
        });
        let encoder1 = image::png::PNGEncoder::new(img_file1);
        encoder1.encode(&mut *img1.raw_pixels(), img1.width(), img1.height(), img1.color()).unwrap_or_else(|error| {
            eprintln!("Error when encoding file : {}", error);
            std::process::exit(-1);
        });

        let img_file2 = std::fs::File::create(format!("{:04}.png", frame)).unwrap_or_else(|error| {
            eprintln!("Error when creating file : {}", error);
            std::process::exit(-1);
        });
        let encoder2 = image::png::PNGEncoder::new(img_file2);
        encoder2.encode(&mut *img2.raw_pixels(), img2.width(), img2.height(), img2.color()).unwrap_or_else(|error| {
            eprintln!("Error when encoding file : {}", error);
            std::process::exit(-1);
        });
    }

    if write_json{
        let mut json_file : File = File::create("output.json").unwrap_or_else(|error| {
            eprintln!("Error when creating file : {}", error);
            std::process::exit(-1);
        });
        let mut json = String::new();
        json.push_str("{\n\t\"name\": \"output\",\n\t\"loops\": 0,\n\t\"skip_first\": false,\n\t\"frames\": [\n");
        json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", 0, 1000).as_str());
        for i in 1..frame as u32 {
            json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", i, t * 10.0).as_str());
        }
        json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", frame, 1000).as_str());
        for i in 1..(frame - 1.0) as u32 {
            json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", frame + i as f32, t * 10.0).as_str());
        }
        json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}}\n", frame + frame - 1.0, t * 10.0).as_str());
        json.push_str("\t]\n");
        json.push_str("}");
        json_file.write(json.as_bytes()).unwrap();
    }

    if img1.width() > std::u16::MAX.into() {
        eprintln!("Error : Width must be <= at {}" , std::u16::MAX);
        std::process::exit(-1);
    }

    if img1.height() > std::u16::MAX.into() {
        eprintln!("Error : Height must be <= at {}" , std::u16::MAX);
        std::process::exit(-1);
    }

    if img1.width() != img2.width() || img1.height() != img2.height() {
        eprint!("Error : Images doesn't have the same dimension");
        std::process::exit(-1);
    }


    let mut output_file = std::fs::File::create(output).unwrap_or_else(|error| {
        eprintln!("Error when creating file : {}", error);
        std::process::exit(-1);
    });

    let mut encoder = Encoder::new(&mut output_file, img1.width() as u16, img2.height() as u16, &[]).unwrap_or_else(|error| {
        eprintln!("Error when creating encoder : {}", error);
        std::process::exit(-1);
    });

    encoder.set(Repeat::Infinite).unwrap();

    let mut frames : Vec<Frame> = Vec::with_capacity((frame+frame) as usize);
    
    println!("Converting Frame 1");
    let mut frame1 = Frame::from_rgb_speed(img1.width() as u16, img1.height() as u16, &mut *img1.raw_pixels(), conversion_speed);
    frame1.delay = 100;

    frames.resize((frame+frame) as usize, frame1);

    println!("Converting Frame {}", frame);
    let mut frame2 = Frame::from_rgb_speed(img2.width() as u16, img2.height() as u16, &mut *img2.raw_pixels(), conversion_speed);
    frame2.delay = 100;
    frames[frame as usize] = frame2;

    for alpha in 1..(frame as u32)  {
        print!("\rGenrating frame {:04} out of {:04}", alpha + 1, frame);
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut img = DynamicImage::new_rgb8(img1.width(), img1.height());
        let a = 1.0 - (alpha as f32/frame);
        for x in 0..img1.width() {
            for y in 0..img1.height() {
                let mut pixel1 = img1.get_pixel(x, y);
                let pixel2 = img2.get_pixel(x, y);

                // TODO OPTIMISE
                pixel1[0] = (((pixel1[0] as f32/255.0)*a + (pixel2[0] as f32/255.0)*(1.0-a)) * 255.0) as u8;
                pixel1[1] = (((pixel1[2] as f32/255.0)*a + (pixel2[1] as f32/255.0)*(1.0-a)) * 255.0) as u8;
                pixel1[2] = (((pixel1[2] as f32/255.0)*a + (pixel2[2] as f32/255.0)*(1.0-a)) * 255.0) as u8;
                img.put_pixel(x, y, pixel1);
            }
        }
        if write_to_disk {
            let img_file1 = std::fs::File::create(format!("{:04}.png", alpha)).unwrap_or_else(|error| {
                eprintln!("Error when creating file : {}", error);
                std::process::exit(-1);
            });
            let encoder1 = image::png::PNGEncoder::new(img_file1);
            encoder1.encode(&mut *img.raw_pixels(), img.width(), img.height(), img.color()).unwrap_or_else(|error| {
                eprintln!("Error when encoding file : {}", error);
                std::process::exit(-1);
            });

            let img_file2 = std::fs::File::create(format!("{:04}.png", frame + frame - alpha as f32)).unwrap_or_else(|error| {
                eprintln!("Error when creating file : {}", error);
                std::process::exit(-1);
            });
            let encoder2 = image::png::PNGEncoder::new(img_file2);
            encoder2.encode(&mut *img.raw_pixels(), img.width(), img.height(), img.color()).unwrap_or_else(|error| {
                eprintln!("Error when encoding file : {}", error);
                std::process::exit(-1);
            });
        }
        
        let mut f = Frame::from_rgb_speed(img1.width() as u16, img1.height() as u16, &mut *img.raw_pixels(), conversion_speed);
        f.delay = t as u16;
        frames[alpha as usize] = Clone::clone(&f);
        frames[(frame + frame - alpha as f32) as usize] = f;
    }

    println!("\nEncoding gif ... ");

    for (i, f) in frames.iter().enumerate() {
        print!("\rWriting frame {:04} out of {:04}", i + 1, frames.len());
        io::stdout().flush().ok().expect("Could not flush stdout");
        encoder.write_frame(&f).unwrap();
    }
    println!("\nDone !");
}