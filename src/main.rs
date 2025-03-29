pub mod augmenters;
pub mod pipeline;
pub mod traits;

use std::{
    error::Error,
    fs::{self, create_dir_all},
    io::Cursor,
    path::Path,
};

use augmenters::{BlurAugmenter, ColorAugmenter, GeometricAugmenter};
use image::{ImageFormat, ImageReader};
use iterators_extended::stateful_threaded::IntoStatefulThreadedIterator;
use pipeline::ThreadsafeAugmenterPipeline;
use traits::Augmenter;

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("/mnt/c/Users/Adam/Pictures/santamancer.png");
    let output_folder = Path::new("/mnt/e/programming/rust/data_processing_pipeline/augmented");
    create_dir_all(output_folder).unwrap();

    let image = ImageReader::open(image_path)?.decode()?;

    let pipeline = ThreadsafeAugmenterPipeline {
        augmenters: vec![
            Box::new(GeometricAugmenter),
            Box::new(ColorAugmenter::new(Some(0.1), Some(10), Some(10))),
            Box::new(BlurAugmenter::new(3.)),
        ],
    };

    let pipeline = ThreadsafeAugmenterPipeline {
        augmenters: vec![Box::new(pipeline.clone()), Box::new(pipeline)],
    };

    (0..1000)
        .map(|_| image.clone())
        .enumerate()
        .stateful_par_map(
            |(pipeline, buf), (i, img)| {
                let img = pipeline.augment_inplace(img);

                // Encode to bytes
                buf.clear();
                let mut buf = Cursor::new(buf);
                img.write_to(&mut buf, ImageFormat::Png).unwrap();

                // Write
                fs::write(output_folder.join(format!("{i}.png")), buf.into_inner()).unwrap();
                println!("{i}");
            },
            (pipeline, vec![]),
        )
        .count();

    Ok(())
}
