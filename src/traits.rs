use dyn_clone::DynClone;
use image::DynamicImage;

/// Image augmenter
pub trait Augmenter {
    /// Augment the image without modifying the source
    fn augment(&self, img: &DynamicImage) -> DynamicImage;

    /// Augment the image, re-using the allocated source image if possible for efficiency
    fn augment_inplace(&self, img: DynamicImage) -> DynamicImage;
}

/// An augmenter that can be easily cloned & shared across threads.
pub trait ClonableAugmenter: Augmenter + DynClone + Sync + Send {}

impl<T> ClonableAugmenter for T where T: Augmenter + Clone + Sync + Send {}
