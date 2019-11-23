use image::{GenericImageView, GenericImage, DynamicImage};
use gif::{Frame, Encoder, Repeat, SetParameter};
use std::{io, io::Write, fs::File};

const RGBA : image::ColorType = image::ColorType::RGBA(8);

// TODO Add support for more than 2 images
fn main() {
    let mut _file1 = None;
    let mut _file2 = None;

    let mut write_to_disk = false;
    let mut write_json = false;
    
    let mut output : String = String::from("output.gif");
    let mut output_dir: String  = String::from("");

    let mut frame_count : u32 = 10;
    let mut frame_duration : f32 = 1000.0 / 10.0;

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
                        output = args[i + 1].clone();
                    } else {
                        eprintln!("Error : Missing output path. Ex : fade image1.jpg image2.jpg -o output.gif");
                        std::process::exit(-1);
                    }
                },
                "-w" => write_to_disk = true,
                "-n" => {
                    if args.len() > i + 1 {
                        frame_count =  args[i + 1].parse::<u32>().unwrap();
                        frame_duration = 1000.0 / frame_count as f32;
                    } else {
                        eprintln!("Error : Missing number of frames. Ex : fade image1.jpg image2.jpg -n 20 will produce a gif with 20 frames per images.");
                        std::process::exit(-1);
                    }
                },
                "-s" => {
                    if args.len() > i + 1 {
                        conversion_speed =  args[i + 1].parse::<i32>().unwrap();
                    } else {
                        eprintln!("Error : Missing speed of conversion. Ex : fade image1.jpg image2.jpg -s 30");
                        std::process::exit(-1);
                    }
                },
                "-r" => {
                    if args.len() > i + 2 {
                        resize = true;
                        resize_width =  Some(args[i + 1].parse::<u32>().unwrap());
                        resize_height = Some(args[i + 2].parse::<u32>().unwrap())
                    } else {
                        eprintln!("Error : Missing resize width and height. Ex : fade image1.jpg image2.jpg -r 320 180");
                        std::process::exit(-1);
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
            eprintln!("Error : File 1 must be set");
            std::process::exit(-1);
        },
    };

    let file2 : &str = match _file2 {
        Some(x) => x,
        None => {
            eprintln!("Error : File 2 must be set"); 
            std::process::exit(-1);
        },
    };

    if !write_to_disk && write_json {
        eprintln!("Warning : You write duration to disk but not frames ?");
    }

    if output != "output.gif" {
        output = output.replace("\\", "/");
        if output.ends_with("/") {
            output_dir = output;
            output = format!("{}output.gif", output_dir);
            //output = format!("{}output.gif", outputDir).to_string()
        } else if output.contains("/") {
            let p : Vec<&str> = output.split("/").collect();
            output_dir = p[0..p.len() - 1].join("/");
            output_dir.push('/');
        }
    }

    print!("Parameters :\n");
    print!("\tFile 1 : {}\n", file1);
    print!("\tFile 2 : {}\n", file2);
    print!("\tOutput : {}\n", output);
    if output_dir != "" {
        print!("\tOutput directory : {}\n", output_dir);
    }
    print!("\tTotal of frames : {}, with a delay of : {}ms\n", frame_count, frame_duration);
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

    if output_dir != "" {
        let path = std::path::Path::new(&output_dir);
        if !path.exists() {
            std::fs::create_dir(path).unwrap_or_else(|error| {
                eprintln!("Error when creating file : {}", error);
                std::process::exit(-1);
            });
            println!("Created {} directory", output_dir);
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
        img1 = img1.resize_exact(resize_width.unwrap(), resize_height.unwrap(), image::imageops::FilterType::Nearest);

        img2 = img2.resize_exact(resize_width.unwrap(), resize_height.unwrap(), image::imageops::FilterType::Nearest);
    }

    if write_to_disk {
        let img_file1 = std::fs::File::create(format!("{}0000.png", output_dir)).unwrap_or_else(|error| {
            eprintln!("Error when creating file : {}", error);
            std::process::exit(-1);
        });
        let encoder1 = image::png::PNGEncoder::new(img_file1);
        encoder1.encode(&mut *img1.raw_pixels(), img1.width(), img1.height(), img1.color()).unwrap_or_else(|error| {
            eprintln!("Error when encoding file : {}", error);
            std::process::exit(-1);
        });

        let img_file2 = std::fs::File::create(format!("{}{:04}.png", output_dir, frame_count)).unwrap_or_else(|error| {
            eprintln!("Error when creating file : {}", error);
            std::process::exit(-1);
        });
        let encoder2 = image::png::PNGEncoder::new(img_file2);
        encoder2.encode(&mut *img2.raw_pixels(), img2.width(), img2.height(), img2.color()).unwrap_or_else(|error| {
            eprintln!("Error when encoding file : {}", error);
            std::process::exit(-1);
        });
    }

    if write_json {
        write_json_to_disk(&output_dir, &frame_count, &frame_duration).unwrap_or_else(|error| {
            eprintln!("Error when writing json : {}", error);
            std::process::exit(-1);
        });
        println!("Writed json to disk");
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
        eprint!("Error : Images doesn't have the same dimension, {} x {}, {} x {}", img1.width(), img1.height(), img2.width(), img2.height());
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

    println!("Converting Frame 1"); 
    let mut frame1 : Frame;
    if img1.color() == RGBA {
        frame1 = Frame::from_rgba_speed(img1.width() as u16, img1.height() as u16, &mut *img1.raw_pixels(), conversion_speed);
    } else {
        frame1 = Frame::from_rgb_speed(img1.width() as u16, img1.height() as u16, &mut *img1.raw_pixels(), conversion_speed);
    };
    frame1.delay = 100;
    
    let mut frames : Vec<Frame> = vec![frame1; (frame_count + frame_count) as usize];

    println!("Converting Frame {}", frame_count);
    let mut frame2 : Frame;
    if img2.color() == RGBA {
        frame2 = Frame::from_rgba_speed(img2.width() as u16, img2.height() as u16, &mut *img2.raw_pixels(), conversion_speed);
    } else {
        frame2 = Frame::from_rgb_speed(img2.width() as u16, img2.height() as u16, &mut *img2.raw_pixels(), conversion_speed);
    };
    frame2.delay = 100;
    frames[frame_count as usize] = frame2;

    if img1.color() == RGBA || img2.color() == RGBA {
        for alpha in 1..(frame_count as u32)  {
            print!("\rGenrating frame {:04} out of {:04}", alpha + 1, frame_count);
            io::stdout().flush().ok().expect("Could not flush stdout");
            let mut img = DynamicImage::new_rgba8(img1.width(), img1.height());
            let a = 0xff - (alpha * 0xff) / frame_count;

            for x in 0..img1.width() {
                for y in 0..img1.height() {
                    let mut pixel1 = img1.get_pixel(x, y);
                    let pixel2 = img2.get_pixel(x, y);

                    pixel1[0] = ((pixel1[0] as u32 * pixel1[3] as u32 * a + pixel2[0] as u32 * pixel2[3] as u32 * (0xff - a)) / 0xfe01) as u8;
                    pixel1[1] = ((pixel1[1] as u32 * pixel1[3] as u32 * a + pixel2[1] as u32 * pixel2[3] as u32 * (0xff - a)) / 0xfe01) as u8;
                    pixel1[2] = ((pixel1[2] as u32 * pixel1[3] as u32 * a + pixel2[2] as u32 * pixel2[3] as u32 * (0xff - a)) / 0xfe01) as u8;
                    pixel1[3] = ((pixel1[3] as u32 * a + pixel2[3] as u32 * (0xff - a as u32)) / 0xff) as u8;

                    img.put_pixel(x, y, pixel1);
                }
            }

            if write_to_disk {
                let img_file1 = std::fs::File::create(format!("{}{:04}.png", output_dir, alpha)).unwrap_or_else(|error| {
                    eprintln!("Error when creating file : {}", error);
                    std::process::exit(-1);
                });
                let encoder1 = image::png::PNGEncoder::new(img_file1);
                encoder1.encode(&mut *img.raw_pixels(), img.width(), img.height(), img.color()).unwrap_or_else(|error| {
                    eprintln!("Error when encoding file : {}", error);
                    std::process::exit(-1);
                });

                let img_file2 = std::fs::File::create(format!("{}{:04}.png", output_dir, frame_count + frame_count - alpha)).unwrap_or_else(|error| {
                    eprintln!("Error when creating file : {}", error);
                    std::process::exit(-1);
                });
                let encoder2 = image::png::PNGEncoder::new(img_file2);
                encoder2.encode(&mut *img.raw_pixels(), img.width(), img.height(), img.color()).unwrap_or_else(|error| {
                    eprintln!("Error when encoding file : {}", error);
                    std::process::exit(-1);
                });
            }
            let mut f = Frame::from_rgba_speed(img1.width() as u16, img1.height() as u16, &mut *img.raw_pixels(), conversion_speed);
            f.delay = (frame_duration / 10.0) as u16;
            frames[alpha as usize] = Clone::clone(&f);
            frames[(frame_count + frame_count - alpha) as usize] = f;
        }
    } else {
        for alpha in 1..(frame_count as u32)  {
            print!("\rGenrating frame {:04} out of {:04}", alpha + 1, frame_count);
            io::stdout().flush().ok().expect("Could not flush stdout");

            let mut img = DynamicImage::new_rgb8(img1.width(), img1.height());
            let a = 0xff - (alpha * 0xff) / frame_count;

            for x in 0..img1.width() {
                for y in 0..img1.height() {
                    let mut pixel1 = img1.get_pixel(x, y);
                    let pixel2 = img2.get_pixel(x, y);

                    pixel1[0] = ((pixel1[0] as u32 * a + pixel2[0] as u32 * (0xff - a)) / 0xff) as u8;
                    pixel1[1] = ((pixel1[1] as u32 * a + pixel2[1] as u32 * (0xff - a)) / 0xff) as u8;
                    pixel1[2] = ((pixel1[2] as u32 * a + pixel2[2] as u32 * (0xff - a)) / 0xff) as u8;
                    
                    img.put_pixel(x, y, pixel1);
                }
            }

            if write_to_disk {
                let img_file1 = std::fs::File::create(format!("{}{:04}.png", output_dir, alpha)).unwrap_or_else(|error| {
                    eprintln!("Error when creating file : {}", error);
                    std::process::exit(-1);
                });
                let encoder1 = image::png::PNGEncoder::new(img_file1);
                encoder1.encode(&mut *img.raw_pixels(), img.width(), img.height(), img.color()).unwrap_or_else(|error| {
                    eprintln!("Error when encoding file : {}", error);
                    std::process::exit(-1);
                });

                let img_file2 = std::fs::File::create(format!("{}{:04}.png", output_dir, frame_count + frame_count - alpha)).unwrap_or_else(|error| {
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
            f.delay = (frame_duration / 10.0) as u16;
            frames[alpha as usize] = Clone::clone(&f);
            frames[(frame_count + frame_count - alpha) as usize] = f;
        }
    }

    println!("\nEncoding gif ... ");

    for (i, f) in frames.iter().enumerate() {
        print!("\rWriting frame {:04} out of {:04}", i + 1, frames.len());
        io::stdout().flush().ok().expect("Could not flush stdout");
        encoder.write_frame(&f).unwrap();
    }
    println!("\nDone !");
}

fn write_json_to_disk(output_dir : &String, frame_count : &u32, frame_duration : &f32) -> Result<(), io::Error> {
    let json_file = File::create(format!("{}animation.json", output_dir));
    let mut json_file = match json_file {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut json = String::new();
    json.push_str("{\n\t\"name\": \"output\",\n\t\"loops\": 0,\n\t\"skip_first\": false,\n\t\"frames\": [\n");
    json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", 0, 1000).as_str());
    for i in 1..*frame_count {
        json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", i, frame_duration).as_str());
    }
    json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", frame_count, 1000).as_str());
    for i in 1..(*frame_count - 1) {
        json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}},\n", frame_count + i, frame_duration).as_str());
    }
    json.push_str(format!("\t\t{{\"{:04}\": \"{}/1000\"}}\n", frame_count + frame_count - 1, frame_duration).as_str());
    json.push_str("\t]\n");
    json.push_str("}");
    let writed = json_file.write(json.as_bytes());
    match writed {
        Ok(_) => return Ok(()),
        Err(x) => return Err(x),
    }
}