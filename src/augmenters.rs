use std::ops::RangeInclusive;

use image::{DynamicImage, metadata::Orientation};
use rand::{random_bool, random_range};

use crate::traits::Augmenter;

/// Horizontal / vertical flips
#[derive(Clone)]
pub struct GeometricAugmenter;

impl Augmenter for GeometricAugmenter {
    fn augment(&self, img: &DynamicImage) -> DynamicImage {
        self.augment_inplace(img.clone())
    }

    fn augment_inplace(&self, mut img: DynamicImage) -> DynamicImage {
        if random_bool(0.5) {
            img.apply_orientation(Orientation::FlipVertical);
        }

        if random_bool(0.5) {
            img.apply_orientation(Orientation::FlipHorizontal);
        }
        img
    }
}

#[derive(Clone)]
pub struct ColorAugmenter {
    contrast_range: Option<RangeInclusive<f32>>,
    hue_range: Option<RangeInclusive<i32>>,
    brightness_range: Option<RangeInclusive<i32>>,
}

impl ColorAugmenter {
    pub fn new(contrast: Option<f32>, hue: Option<i32>, brightness: Option<i32>) -> Self {
        let contrast_range = contrast.map(|c| {
            assert!(c > 0.);
            -c..=c
        });

        let hue_range = hue.map(|h| {
            assert!(h > 0);
            -h..=h
        });

        let brightness_range = brightness.map(|b| {
            assert!(b > 0);
            -b..=b
        });

        Self {
            contrast_range,
            hue_range,
            brightness_range,
        }
    }
}

impl Augmenter for ColorAugmenter {
    fn augment(&self, img: &DynamicImage) -> DynamicImage {
        let mut img = img.clone();
        if let Some(range) = self.contrast_range.clone() {
            img = img.adjust_contrast(random_range(range));
        }
        if let Some(range) = self.hue_range.clone() {
            img = img.huerotate(random_range(range));
        }
        if let Some(range) = self.brightness_range.clone() {
            img = img.brighten(random_range(range));
        }

        img
    }

    fn augment_inplace(&self, img: DynamicImage) -> DynamicImage {
        self.augment(&img)
    }
}

#[derive(Clone)]
pub struct BlurAugmenter {
    sigma_range: RangeInclusive<f32>,
}

impl BlurAugmenter {
    pub fn new(max_sigma: f32) -> Self {
        assert!(max_sigma > 0.);
        Self {
            sigma_range: 0_f32..=max_sigma,
        }
    }
}

impl Augmenter for BlurAugmenter {
    fn augment(&self, img: &DynamicImage) -> DynamicImage {
        img.fast_blur(random_range(self.sigma_range.clone()))
    }

    fn augment_inplace(&self, img: DynamicImage) -> DynamicImage {
        self.augment(&img)
    }
}
