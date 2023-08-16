use gif::Encoder;
use image::{
    codecs::gif::Repeat, Delay, DynamicImage, Frame, GenericImage, GenericImageView, ImageEncoder,
};
use std::{fs::File, io, io::Write, time::Duration};

const RGBA: image::ColorType = image::ColorType::Rgba8;

fn main() {
    let mut write_to_disk = false;
    let mut write_json = false;

    let mut output: String = String::from("output.gif");
    let mut output_dir: String = String::from("");

    let mut frame_count: u32 = 10;
    let mut frame_duration: f32 = 1000.0 / 10.0;
    let mut important_frame_duration: f32 = 1000.0;
    let mut lock_duration = false;

    let mut resize = false;
    let mut resize_width = None;
    let mut resize_height = None;

    let mut conversion_speed: i32 = 10;

    let args: Vec<String> = std::env::args().collect();

    let mut is_images_arg = true;
    let mut images: Vec<String> = Vec::new();

    if args.len() == 1 {
        println!("Usage : fade <file 1> <file 2> [options]");
        println!("Options :");
        println!("\t-o <output path> Set output path.");
        println!("\t-w Write frames to disk.");
        println!("\t-a Write a .json used by apngasm.");
        println!("\t-n <count> Set frames count.");
        println!("\t-d <important> <standard> Set durations of frame in ms");
        println!("\t-s <speed> Set gif conversion speed. Must be between 1 and 30, 30 is loss quality but faster.");
        println!("\t-r <width> <height> Resize image.");
        println!("\nExamples :");
        println!("\tfade image1.jpg image2.jpg will create an animation from the 2 images");
        println!("\tfade *.png -o o.gif -n 50 will take every images in the directory that end with .png, output the result to o.gif and with 50 frames per images");
        std::process::exit(0);
    }

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-o" => {
                if args.len() > i + 1 {
                    output = args[i + 1].clone();
                } else {
                    eprintln!("Error : Missing output path. Ex : fade image1.jpg image2.jpg -o output.gif");
                    std::process::exit(-1);
                }
                is_images_arg = false;
            }
            "-w" => {
                write_to_disk = true;
                is_images_arg = false;
            }
            "-n" => {
                if args.len() > i + 1 {
                    frame_count = args[i + 1].parse::<u32>().unwrap();
                    if !lock_duration {
                        frame_duration = 1000.0 / frame_count as f32;
                    }
                } else {
                    eprintln!("Error : Missing number of frames. Ex : fade image1.jpg image2.jpg -n 20 will produce a gif with 20 frames per images.");
                    std::process::exit(-1);
                }
                is_images_arg = false;
            }
            "-s" => {
                if args.len() > i + 1 {
                    conversion_speed = args[i + 1].parse::<i32>().unwrap();
                } else {
                    eprintln!("Error : Missing speed of conversion. Ex : fade image1.jpg image2.jpg -s 30");
                    std::process::exit(-1);
                }
                is_images_arg = false;
            }
            "-r" => {
                if args.len() > i + 2 {
                    resize = true;
                    resize_width = Some(args[i + 1].parse::<u32>().unwrap());
                    resize_height = Some(args[i + 2].parse::<u32>().unwrap())
                } else {
                    eprintln!("Error : Missing resize width and height. Ex : fade image1.jpg image2.jpg -r 320 180");
                    std::process::exit(-1);
                }
                is_images_arg = false;
            }
            "-a" => {
                write_json = true;
                is_images_arg = false;
            }
            "-d" => {
                if args.len() > i + 2 {
                    lock_duration = true;
                    important_frame_duration = args[i + 1].parse::<f32>().unwrap();
                    frame_duration = args[i + 2].parse::<f32>().unwrap();
                } else {
                    eprintln!(
                        "Error : Missing durations. Ex : fade image1.jpg image2.jpg -d 1000 10"
                    );
                    std::process::exit(-1);
                }
                is_images_arg = false;
            }
            _ => {
                if is_images_arg && i > 0 {
                    images.push(arg.to_string());
                }
            }
        }
    }

    if images.len() == 1 {
        if images[0].contains("*") {
            let end_name = images[0].replace("*", "");
            images.clear();
            let current_dir = std::env::current_dir().unwrap();
            for f in std::fs::read_dir(current_dir).unwrap() {
                let entry: std::fs::DirEntry = f.unwrap();
                let name = entry.file_name().into_string().unwrap();
                if name.ends_with(end_name.as_str()) {
                    images.push(name);
                }
            }
        }
    }

    if images.len() == 0 {
        eprintln!("No images provided !");
        std::process::exit(-1);
    }

    if !write_to_disk && write_json {
        eprintln!("Warning : You write apngasm json to disk but not frames ?");
    }

    if output != "output.gif" {
        output = output.replace("\\", "/");
        if output.ends_with("/") {
            output_dir = output;
            output = format!("{}output.gif", output_dir);
        } else if output.contains("/") {
            let p: Vec<&str> = output.split("/").collect();
            output_dir = p[0..p.len() - 1].join("/");
            output_dir.push('/');
        }
    }

    print!("Parameters :\n");
    print!("\tImages  : {}\n", images.join(", "));
    print!("\tOutput : {}\n", output);
    if output_dir != "" {
        print!("\tOutput directory : {}\n", output_dir);
    }
    print!(
        "\tTotal of frames : {}\n",
        frame_count * images.len() as u32
    );
    print!(
        "\tDuration of standard frames : {}ms, duration of important frames : {}ms\n",
        frame_duration, important_frame_duration
    );
    print!("\tWrite frames to disk : {}\n", write_to_disk);
    print!("\tWrite .json for apngasm : {}\n", write_json);
    print!("\tSpeed of conversion : {}\n", conversion_speed);
    if resize {
        print!(
            "\tResize : true, width : {}, height : {}\n\n",
            resize_width.unwrap(),
            resize_height.unwrap()
        );
    } else {
        print!("\tResize : false\n\n");
    }
    io::stdout().flush().ok().expect("Could not flush stdout");

    if output_dir != "" {
        let path = std::path::Path::new(&output_dir);
        if !path.exists() {
            std::fs::create_dir(path).unwrap_or_else(|error| {
                eprintln!("Error when creating file : {}", error);
                std::process::exit(-1);
            });
            println!("Created {} directory\n", output_dir);
        }
    }

    if write_json {
        write_json_to_disk(
            &output_dir,
            &(images.len() as u32),
            &frame_count,
            &important_frame_duration,
            &frame_duration,
        )
        .unwrap_or_else(|error| {
            eprintln!("Error when writing json : {}", error);
            std::process::exit(-1);
        });
        println!("Writed json to disk\n");
    }

    let mut image_list: Vec<DynamicImage> = Vec::new();

    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for (i, f) in images.iter().enumerate() {
        println!("Opening {}", f);
        let path = std::path::Path::new(f.as_str());
        if !path.exists() {
            eprintln!("Error : File {} doesn't exist !", f);
            std::process::exit(-1)
        } else if path.is_dir() {
            eprintln!("Error File 1 {} is a directory", f);
            std::process::exit(-1);
        }

        let mut img: DynamicImage = image::open(path).unwrap_or_else(|error| {
            eprintln!("Error when decoding img1 : {}", error);
            std::process::exit(-1);
        });

        if img.width() > std::u16::MAX.into() {
            eprintln!("Error : Width must be <= at {}", std::u16::MAX);
            std::process::exit(-1);
        }

        if img.height() > std::u16::MAX.into() {
            eprintln!("Error : Height must be <= at {}", std::u16::MAX);
            std::process::exit(-1);
        }

        if resize {
            img = img.resize_exact(
                resize_width.unwrap(),
                resize_height.unwrap(),
                image::imageops::FilterType::Nearest,
            );
        }

        if i == 0 {
            width = img.width();
            height = img.height();
        } else {
            if img.width() != width || img.height() != height {
                eprint!(
                    "Error : Images doesn't have the same dimension, {} x {}, {} x {}",
                    width,
                    height,
                    img.width(),
                    img.height()
                );
                std::process::exit(-1);
            }
        }

        if write_to_disk {
            let img_file = img
                .save(format!("{}{:04}.png", output_dir, frame_count as usize * i))
                .expect("Failed to save file");
        }

        image_list.push(img)
    }
    println!();

    let mut output_file = std::fs::File::create(output).unwrap_or_else(|error| {
        eprintln!("Error when creating file : {}", error);
        std::process::exit(-1);
    });

    let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(&mut output_file, conversion_speed);

    encoder.set_repeat(Repeat::Infinite).unwrap();

    let lenght_list = image_list.len();

    for i in 0..lenght_list {
        let img1 = &image_list[i];
        print!(
            "\rCreating and writing frame {:04} out of {:04}",
            frame_count * i as u32 + 1,
            frame_count * lenght_list as u32
        );
        io::stdout().flush().ok().expect("Could not flush stdout");

        {
            encoder
                .encode_frame(Frame::from_parts(
                    img1.clone().into_rgba8(),
                    0,
                    0,
                    Delay::from_saturating_duration(Duration::from_millis(
                        important_frame_duration as u64,
                    )),
                ))
                .unwrap()
        }

        let img2: &DynamicImage;
        if i == (lenght_list - 1) {
            img2 = &image_list[0];
        } else {
            img2 = &image_list[i + 1];
        }

        if img1.color() == RGBA || img2.color() == RGBA {
            for alpha in 1..(frame_count as u32) {
                print!(
                    "\rCreating and writing frame {:04} out of {:04}",
                    frame_count * i as u32 + alpha + 1,
                    frame_count * lenght_list as u32
                );
                io::stdout().flush().ok().expect("Could not flush stdout");

                let mut img = DynamicImage::new_rgba8(img1.width(), img1.height());
                let a = 0xff - (alpha * 0xff) / frame_count;

                for x in 0..img1.width() {
                    for y in 0..img1.height() {
                        let mut pixel1 = img1.get_pixel(x, y);
                        let pixel2 = img2.get_pixel(x, y);

                        pixel1[0] = ((pixel1[0] as u32 * pixel1[3] as u32 * a
                            + pixel2[0] as u32 * pixel2[3] as u32 * (0xff - a))
                            / 0xfe01) as u8;
                        pixel1[1] = ((pixel1[1] as u32 * pixel1[3] as u32 * a
                            + pixel2[1] as u32 * pixel2[3] as u32 * (0xff - a))
                            / 0xfe01) as u8;
                        pixel1[2] = ((pixel1[2] as u32 * pixel1[3] as u32 * a
                            + pixel2[2] as u32 * pixel2[3] as u32 * (0xff - a))
                            / 0xfe01) as u8;
                        pixel1[3] = ((pixel1[3] as u32 * a + pixel2[3] as u32 * (0xff - a as u32))
                            / 0xff) as u8;

                        img.put_pixel(x, y, pixel1);
                    }
                }

                if write_to_disk {
                    img.save(format!(
                        "{}{:04}.png",
                        output_dir,
                        frame_count * i as u32 + alpha
                    ))
                    .expect("Failed to save file");
                }
                encoder
                    .encode_frame(Frame::from_parts(
                        img.to_rgba8(),
                        0,
                        0,
                        Delay::from_saturating_duration(Duration::from_millis(
                            frame_duration as u64,
                        )),
                    ))
                    .unwrap();
            }
        } else {
            for alpha in 1..(frame_count as u32) {
                print!(
                    "\rCreating and writing frame {:04} out of {:04}",
                    frame_count * i as u32 + alpha + 1,
                    frame_count * lenght_list as u32
                );
                io::stdout().flush().ok().expect("Could not flush stdout");

                let mut img = DynamicImage::new_rgb8(img1.width(), img1.height());
                let a = 0xff - (alpha * 0xff) / frame_count;

                for x in 0..img1.width() {
                    for y in 0..img1.height() {
                        let mut pixel1 = img1.get_pixel(x, y);
                        let pixel2 = img2.get_pixel(x, y);

                        pixel1[0] =
                            ((pixel1[0] as u32 * a + pixel2[0] as u32 * (0xff - a)) / 0xff) as u8;
                        pixel1[1] =
                            ((pixel1[1] as u32 * a + pixel2[1] as u32 * (0xff - a)) / 0xff) as u8;
                        pixel1[2] =
                            ((pixel1[2] as u32 * a + pixel2[2] as u32 * (0xff - a)) / 0xff) as u8;

                        img.put_pixel(x, y, pixel1);
                    }
                }

                if write_to_disk {
                    img.save(format!(
                        "{}{:04}.png",
                        output_dir,
                        frame_count * i as u32 + alpha
                    ))
                    .expect("Failed to save to file");
                }

                encoder
                    .encode_frame(Frame::from_parts(
                        img.to_rgba8(),
                        0,
                        0,
                        Delay::from_saturating_duration(Duration::from_millis(
                            frame_duration as u64,
                        )),
                    ))
                    .unwrap();
            }
        }
    }
    println!("\nDone !");
}

fn write_json_to_disk(
    output_dir: &String,
    images_count: &u32,
    frame_count: &u32,
    important_frame_duration: &f32,
    frame_duration: &f32,
) -> Result<(), io::Error> {
    let json_file = File::create(format!("{}animation.json", output_dir));
    let mut json_file = match json_file {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let json = generate_json(
        images_count,
        frame_count,
        important_frame_duration,
        frame_duration,
    );
    let writed = json_file.write(json.as_bytes());
    match writed {
        Ok(_) => return Ok(()),
        Err(x) => return Err(x),
    }
}

fn generate_json(
    images_count: &u32,
    frame_count: &u32,
    important_frame_duration: &f32,
    frame_duration: &f32,
) -> String {
    let mut json = String::new();
    json.push_str(
        "{\n\t\"name\": \"output\",\n\t\"loops\": 0,\n\t\"skip_first\": false,\n\t\"frames\": [\n",
    );
    for f in 0..*images_count {
        json.push_str(
            format!(
                "\t\t{{\"{:04}\": \"{}/1000\"}},\n",
                f * frame_count,
                important_frame_duration
            )
            .as_str(),
        );
        for i in 1..*frame_count {
            json.push_str(
                format!(
                    "\t\t{{\"{:04}\": \"{}/1000\"}},\n",
                    i + f * frame_count,
                    frame_duration
                )
                .as_str(),
            );
        }
    }
    json.remove(json.len() - 1);
    json.remove(json.len() - 1);
    json.push_str("\n\t]\n");
    json.push_str("}");
    return json;
}

#[cfg(test)]
mod test {
    #[test]
    fn json() {
        let json_res = "{\n\t\"name\": \"output\",\n\t\"loops\": 0,\n\t\"skip_first\": false,\n\t\"frames\": [\n\t\t{\"0000\": \"100/1000\"},\n\t\t{\"0001\": \"10/1000\"},\n\t\t{\"0002\": \"10/1000\"},\n\t\t{\"0003\": \"10/1000\"},\n\t\t{\"0004\": \"10/1000\"},\n\t\t{\"0005\": \"10/1000\"},\n\t\t{\"0006\": \"10/1000\"},\n\t\t{\"0007\": \"10/1000\"},\n\t\t{\"0008\": \"10/1000\"},\n\t\t{\"0009\": \"10/1000\"},\n\t\t{\"0010\": \"100/1000\"},\n\t\t{\"0011\": \"10/1000\"},\n\t\t{\"0012\": \"10/1000\"},\n\t\t{\"0013\": \"10/1000\"},\n\t\t{\"0014\": \"10/1000\"},\n\t\t{\"0015\": \"10/1000\"},\n\t\t{\"0016\": \"10/1000\"},\n\t\t{\"0017\": \"10/1000\"},\n\t\t{\"0018\": \"10/1000\"},\n\t\t{\"0019\": \"10/1000\"}\n\t]\n}";
        assert_eq!(json_res, crate::generate_json(&2, &10, &100.0, &10.0));
    }
}
