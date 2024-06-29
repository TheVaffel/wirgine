
use super::image::Image;
use super::log::Log;


use image_compare::{self, Similarity};
use std::path::Path;

const SIMILARITY_THRESHOLD: f64 = 0.98;

pub fn create_or_compare_images(test_name: &str, rendered_images: &Vec<Image<u32>>) -> Result<(), ()>{
    for i in 0..rendered_images.len() {
        let mut image_copy = rendered_images[i].clone();
        image_copy.rgb_shuffle();
        let image_dir = "test/test_images/";
        let original_path = format!("{}{}{}.png", image_dir, test_name, i);
        let image_path = Path::new(&original_path);
        if !image_path.exists() {
            Log::info(&format!("Could not find image {}, creating it", &original_path));
            return write_new_image(&image_path, &image_copy);
        }

        let new_path_name = &format!("{}{}_rendered{}.png", image_dir, test_name, i);
        let new_path = Path::new(&new_path_name);
        let result = image_copy.write_image(&new_path);
        if let Err(_) = result {
            Log::fail(&format!("Could not write image to file {}", new_path_name));
            return Err(());
        }

        let similarity = compare_images(&image_path, &new_path);

        if similarity.score < SIMILARITY_THRESHOLD {

            let deviation_path_name = format!("{}{}_deviation{}.png", image_dir, test_name, i);
            let deviation_path = Path::new(&deviation_path_name);
            Log::fail(&format!("Images were not equal, score was {}, threshold is {}, writing deviation to {}",
                               similarity.score,
                               SIMILARITY_THRESHOLD,
                               get_name(&deviation_path)));

            let _ =  write_deviation_image(&similarity, &deviation_path);
            return Err(())
        }
    }

    Ok(())
}

fn get_name(path: &Path) -> &str {
    path.to_str().unwrap()
}

fn compare_images(path0: &Path, path1: &Path) -> Similarity {
    let old_image = image::open(&path0).expect("Found original image, but could not read").to_rgb8();
    let new_image = image::open(&path1).expect("Could not read image that I just wrote (should never happen)").to_rgb8();

    let compare_result = image_compare::rgb_similarity_structure(&image_compare::Algorithm::MSSIMSimple, &old_image, &new_image);

    return compare_result.expect("Could not compare images");
}

fn write_new_image(image_path: &Path, image: &Image<u32>) -> Result<(), ()> {
    let result = image.write_image(&image_path);
    if let Err(error) = result {
        Log::fail(&format!("Error writing image {}", get_name(&image_path)));
        return Err(());
    }

    Log::success(&format!("Wrote new image {}", get_name(&image_path)));

    Ok(())
}

fn write_deviation_image(similarity: &Similarity, path: &Path) -> Result<(), ()> {

    let deviation_image = similarity.image.to_color_map();
    let write_result = deviation_image.save(&path);
    if let Err(_) = write_result {
        Log::fail(&format!("Could not write deviation image {}", get_name(&path)))
    }

    return Err(())
}
