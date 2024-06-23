use super::image::Image;
use super::log::Log;

use image::load;
use num::Zero;

use image_compare;

use std::path::Path;
use std::fmt::format;

const SIMILARITY_THRESHOLD: f64 = 0.98;

pub fn create_or_compare_images(test_name: &str, rendered_images: &Vec<Image<u32>>) -> Result<(), ()>{
    for i in 0..rendered_images.len() {
        let mut image_copy = rendered_images[i].clone();
        image_copy.rgb_shuffle();
        let original_path = format!("{}{}.png", test_name, i);
        let image_path = Path::new(&original_path);
        if !image_path.exists() {
            Log::info(&format!("Could not find image {}, creating it", &original_path));
            let result = image_copy.write_image(&image_path);
            if let Err(error) = result {
                Log::fail(&format!("Error writing image {}", &original_path));
                return Err(());
            }

            Log::success(&format!("Wrote new image {}", &original_path));
        } else {
            let new_path_name = &format!("{}_rendered{}.png", test_name, i);
            let new_path = Path::new(&new_path_name);
            let result = image_copy.write_image(&new_path);

            let old_image = image::open(&original_path).expect("Found original image, but could not read").to_rgb8();
            let new_image = image::open(&new_path).expect("Could not read image that I just wrote (should never happen)").to_rgb8();

            let compare_result = image_compare::rgb_similarity_structure(&image_compare::Algorithm::MSSIMSimple, &old_image, &new_image);

            let similarity = compare_result.expect("Could not compare images");

            if similarity.score < SIMILARITY_THRESHOLD {
                let deviation_path_name = format!("{}_deviation{}.png", test_name, i);
                let deviation_path = Path::new(&deviation_path_name);

                Log::fail(&format!("Images were not equal, score was {}, threshold is {}, writing deviation to {}", similarity.score, SIMILARITY_THRESHOLD, deviation_path_name));

                let deviation_image = similarity.image.to_color_map();
                let write_result = deviation_image.save(deviation_path);
                if let Err(_) = write_result {
                    Log::fail(&format!("Could not write deviation image {}", deviation_path_name))
                }

                return Err(())
            }
        }
    }

    Ok(())
}
