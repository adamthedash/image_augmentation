use dyn_clone::clone_box;
use image::DynamicImage;

use crate::traits::{Augmenter, ClonableAugmenter};

/// A pipeline of augmenters which can be freely cloned and shared across threads
pub struct ThreadsafeAugmenterPipeline {
    pub augmenters: Vec<Box<dyn ClonableAugmenter>>,
}

impl Clone for ThreadsafeAugmenterPipeline {
    fn clone(&self) -> Self {
        Self {
            augmenters: self.augmenters.iter().map(|a| clone_box(&**a)).collect(),
        }
    }
}

impl Augmenter for ThreadsafeAugmenterPipeline {
    fn augment(&self, img: &DynamicImage) -> DynamicImage {
        self.augment_inplace(img.clone())
    }

    fn augment_inplace(&self, img: DynamicImage) -> DynamicImage {
        self.augmenters
            .iter()
            .fold(img, |img, aug| aug.augment_inplace(img))
    }
}

/// A pipeline of augmenters which are not easily clonable (Eg. need specific resource allocation
/// on setup)
pub struct AugmenterPipeline {
    pub augmenters: Vec<Box<dyn Augmenter>>,
}

impl Augmenter for AugmenterPipeline {
    fn augment(&self, img: &DynamicImage) -> DynamicImage {
        self.augment_inplace(img.clone())
    }

    fn augment_inplace(&self, img: DynamicImage) -> DynamicImage {
        self.augmenters
            .iter()
            .fold(img, |img, aug| aug.augment_inplace(img))
    }
}
