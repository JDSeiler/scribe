use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::process::exit;
use crate::FailureReason::{FileIoFailure, FileTypeMismatch, InvalidRange};
use image::{GrayImage, ImageFormat};

const IMAGE_FILE: u32 = 2051;
const LABEL_FILE: u32 = 2049;
const BYTES_PER_IMAGE: u32 = 28 * 28;

type ImageData = [u8; BYTES_PER_IMAGE as usize];

#[derive(Debug)]
enum FailureReason {
    FileIoFailure,
    FileTypeMismatch,
    InvalidRange
}

type ImageFileWithLabels = (BufReader<File>, BufReader<File>);

fn main() {
    let args = read_args();

    if let Some(args) = args {
        let file_handles = prepare_read_handles(&args).expect("Eek! Something went badly wrong!");
        let res = dump_images(file_handles, args.quantity);
        println!("Operation result: {:#?}", res);
    }
}

fn dump_images(file_handles: ImageFileWithLabels, quantity: u32) -> std::io::Result<()> {
    let (mut source, mut labels) = file_handles;
    let mut num_images_read = 0;
    while num_images_read < quantity {
        let label = read_u8(&mut labels)?;
        let image_data = read_image_data(&mut source)?;

        let img = GrayImage::from_raw(28, 28, image_data);
        match img {
            Some(buf) => {
                let img_destination = format!("./out/img-{}-is-a-{}.bmp", num_images_read, label);
                buf.save_with_format(img_destination, ImageFormat::Bmp).expect("Failed to save image");
            },
            None => {
                eprintln!("Failed to create image buffer from data!")
            }
        }
        num_images_read += 1;
    }

    Ok(())
}

fn prepare_read_handles(args: &Args) -> std::io::Result<ImageFileWithLabels> {
    let mut source = get_read_handle(args.input_source.as_str())?;
    let mut labels = get_read_handle(args.input_labels.as_str())?;

    operation_is_valid(&mut source, IMAGE_FILE, args).unwrap_or_else(|failure_reason| {
        println!("Operation invalid due to issue with images file: {:#?}", failure_reason);
        exit(1);
    });
    operation_is_valid(&mut labels, LABEL_FILE, args).unwrap_or_else(|failure_reason| {
        println!("Operation invalid due to issue with labels file: {:#?}", failure_reason);
        exit(1);
    });

    source.seek(SeekFrom::Start(16))?;
    labels.seek(SeekFrom::Start(8))?;

    let source_read_offset = (args.start_from - 1) * BYTES_PER_IMAGE;
    let label_read_offset = args.start_from - 1;

    source.seek_relative(source_read_offset as i64)?;
    labels.seek_relative(label_read_offset as i64)?;

    Ok((source, labels))
}

fn get_read_handle(path: &str) -> std::io::Result<BufReader<File>> {
    let f = File::open(path)?;
    Ok(BufReader::new(f))
}

fn operation_is_valid(file: &mut BufReader<File>, expected_magic_number: u32, args: &Args) -> std::result::Result<(), FailureReason> {
    let magic_number = match read_u32(file) {
        Ok(mn) => mn,
        Err(_) => { return Err(FileIoFailure); }
    };
    let num_images = match read_u32(file) {
        Ok(num) => num,
        Err(_) => { return Err(FileIoFailure); }
    };

    if magic_number != expected_magic_number {
        Err(FileTypeMismatch)
    } else {
        match (args.quantity >= 1) && (args.start_from >= 1) && (args.start_from + args.quantity - 1) <= num_images {
            false => Err(InvalidRange),
            true => Ok(())
        }
    }
}

fn read_u32(reader: &mut BufReader<File>) -> std::io::Result<u32> {
   let mut u32_bytes: [u8; 4] = [0; 4];
    reader.read_exact(&mut u32_bytes)?;

    Ok(u32::from_be_bytes(u32_bytes))
}

fn read_u8(reader: &mut BufReader<File>) -> std::io::Result<u8> {
    let mut u8_byte: [u8; 1] = [0; 1];
    reader.read_exact(&mut u8_byte)?;

    Ok(u8_byte[0])
}

fn read_image_data(reader: &mut BufReader<File>) -> std::io::Result<Vec<u8>> {
    let mut image_bytes: ImageData = [0; BYTES_PER_IMAGE as usize];
    reader.read_exact(&mut image_bytes)?;

    // MNIST data has 0 as white and 255 as black for some reason.
    // Exactly swapped from "normal"
    Ok(Vec::from(image_bytes).into_iter().map(|b| 255 - b).collect())
}

fn read_args() -> Option<Args> {
    let mut raw_args: Vec<String> = env::args().collect();
    // Discard the first argument, it's the program name and we don't care
    raw_args.remove(0);

    if raw_args.len() == 1 {
        match raw_args.get(0) {
            Some(option) => {
                if option.eq("--help") {
                    print_help_and_exit()
                } else {
                    println!("Unknown option {:#?}", option);
                    print_help_and_exit();
                }
            },
            None => {
                print_help_and_exit();
            }
        }
    } else if raw_args.len() > 1 && raw_args.len() < 4 {
        println!("Missing required arguments!");
        print_help_and_exit();
    } else {
       match &raw_args[..] {
           [source, label, raw_start, raw_quantity] => {
               let start_from: u32 = raw_start.parse::<u32>().expect("Could not parse argument start_from to a u64");
               let quantity: u32 = raw_quantity.parse::<u32>().expect("Could not parse argument quantity to a u64");
               return Some(Args {
                   input_source: source.to_owned(),
                   input_labels: label.to_owned(),
                   start_from,
                   quantity
               });
           },
           _ => {
               eprintln!("Incorrect number of arguments, expected 4, got {:#?}", raw_args.len());
           }
       }
    }
    None
}

fn print_help_and_exit() {
    println!("Scribe, a tool for converting MNIST data into BMP images.");
    println!("Usage:");
    println!("scribe [input_source] [input_labels] [start_from] [quantity]");
    println!("    input_source: Path to the data file to read from");
    println!("    input_labels: Path to the corresponding label file");
    println!("    start_from: Natural number describing which image to start from");
    println!("    quantity: Number of images to read, starting from `start_from`");
    println!("Use the --help option to print this message");
    exit(0);
}

#[derive(Debug)]
struct Args {
    input_source: String,
    input_labels: String,
    start_from: u32,
    quantity: u32
}
