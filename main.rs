extern crate image;

use image::DynamicImage;
use image::GenericImageView;
use std::time::Duration;
use std::process::Command;
use std::env;
use std::fs;

// Oct 31 2023
// NEEDS TO BE SPED UP IN SHOTCUT BY ~0.46400
// BETTER PERFORMANCE HERE IS ALSO OPTIMAL

// Nov 1 2023
// wtf i used build --release and now its in 2x speed...i dont know wether to be happy or frusterated
// i will literaly have to cap it wow...at least i can get playspeed equal to the vid :)

// Nov 7 2023
// F this stupid language! "Cannot be indexed by usize". In python this would be done in a minute not an afternoon
// I have F ing stuff to do F RUST!
// I've beeen trying to get the index of a string to use for a brightness scale and the stupid compiler keeps whining my a** off!
// I swear a 3 year old in mcdonalds would be more tame...
// but hey I got the video speed right so it all evens out :D

// Nov 8 2023
// today I figured out how to set the ratio I thought I would have to average out pixels
// but apparently all I needed to do was to just change how often the for loops ran
// if y / VIDEO_SIZE % != 0 { continue; } thats all it took...I built ratios into it from the begining and i didnt even realize it
// Im either super smart or super stupid...

// Nov 13 2023
// Im currently trying to work out the file issues
// such that the program could generate the frames if needed
// F THIS!!!

// Im going to upload the files to github now wish me luck!

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn get_number_of_files(directory_path: &str) -> Result<u32, std::io::Error> {
let count = fs::read_dir(directory_path)?
    .filter(|entry| entry.as_ref().map(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false)).unwrap_or(false))
    .count() as u32;

Ok(count)
}

fn main() {

    let mut video_size = "8";
    let mut starting_frame = "1";
    let mut ending_frame = "6572";
    let mut ending_frame2 = 0;

    let path_to_frames: &str = &(&format!("{}/frames", get_current_working_dir()) as &str);

    if let Ok(count) = get_number_of_files(path_to_frames) {
        ending_frame2 = count;
    } else {
        return;
    }


    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.len() % 2 == 1 {
        for i in 1..args.len() {
            if i % 2 == 0 {
                continue;
            }
            if args[i].trim() == "video_size".to_string() {
                video_size = &args[i+1];
            }
        }
    }

    if args.len() > 1 && args.len() % 2 == 1 {
        for i in 1..args.len() {
            if i % 2 == 0 {
                continue;
            }
            if args[i].trim() == "starting_frame".to_string() {
                starting_frame = &args[i+1];
            }
        }
    }

    if args.len() > 1 && args.len() % 2 == 1 {
        for i in 1..args.len() {
            if i % 2 == 0 {
                continue;
            }
            if args[i].trim() == "ending_frame".to_string() {
                ending_frame = &args[i+1];
            }
        }
    }

    std::env::set_current_dir(format!("{}/frames", get_current_working_dir()))
        .expect("Unable to change into current working directory");

    if args.len() > 1 && args.len() % 2 == 1 {
        for i in 1..args.len() {
            if i % 2 == 0 {
                continue;
            }
            if args[i].trim() == "find_video".to_string() {

                match fs::metadata(get_current_working_dir()) {
                    
                    Ok(_metadata) => {
                        if fs::metadata(format!("{}/frame_0001.png", get_current_working_dir())).is_ok() {
                            
                            match fs::read_dir(get_current_working_dir()) {
                                Ok(entries) => {
                                    for entry in entries {
                                        if let Ok(entry) = entry {
                                            let file_path = entry.path();
                        
                                            if file_path.is_file() {
                                                match fs::remove_file(&file_path) {
                                                    Ok(_) => {},
                                                    Err(_err) => return,
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(_err) => return,
                            }
                        }
                    }
                    Err(_) => {
                        match fs::create_dir(get_current_working_dir()) {
                            Ok(_) => {},
                            Err(_err) => return,
                        };
                    }
                }

                Command::new("ffmpeg")
                    .arg("-i")
                    .arg(format!("{}", &args[i+1]))
                    .arg("-vf")
                    .arg("fps=30")
                    .arg("frame_%04d.png")
                    .output()
                    .expect("Failed to run ffmpeg");
            }
        }
    }

    let video_size: u32 = match video_size.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let starting_frame: u32 = match starting_frame.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let mut ending_frame: u32 = match ending_frame.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if ending_frame2 != 0 {
        ending_frame = ending_frame2;
    }

    for frame in starting_frame..ending_frame {

        let clear = Command::new("clear")
        .output()
        .expect("ERROR: Failed to execute command");

        /*if frame % 30 == 0 {
            if clear.status.success() {
                let result = String::from_utf8_lossy(&clear.stdout);
                println!("{}", result);
            } else {
                let error = String::from_utf8_lossy(&clear.stderr);
            }    
        }*/

        let image_path = format!(
            "{}/frame_{}.png",
            get_current_working_dir(), 
            format!("{:04}", frame)
        );

        let img: DynamicImage = image::open(image_path).expect("Failed to open image.");

        let w = img.width();
        let h = img.height();

        let mut frame_output = String::new();

        for y in 0..h {

            if y % video_size != 0 {
                continue;
            }

            let mut frame_output_line = String::new();
                
            for x in 0..w {

                if x % video_size / 2 != 0 {
                    continue;
                }

                let avg: u8 = img.get_pixel(x, y)[0]
                .saturating_add(img.get_pixel(x, y)[1])
                .saturating_add(img.get_pixel(x, y)[2]) as u8;

                match avg.min(255) as u8 {
                    avg if avg >= 255 => frame_output_line += "@",
                    avg if avg >= 192 => frame_output_line += "$",
                    avg if avg >= 128 => frame_output_line += "*",
                    
                    avg if avg >= 0 => frame_output_line += " ",
                    _ => return,
                };

            }
            frame_output += &(frame_output_line + "\n");
        
        }
        println!("{}", frame_output);
        
	// My bad math

		// Average frame execution time is ~2-3 milliseconds so that must be taken into account for ~30 fps
		// 30 frames with 2 millis delay = 2 * 30 = 60
		// 1000 - 60 execution time
		// 940 / 30 frames = 31.333...

		// 1000 - (fps * delay) / fps

        std::thread::sleep(Duration::from_millis(31));
    }

}
