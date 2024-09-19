use glam::{Affine2, Vec2};

pub(crate) fn factors(width: f32, height: f32, new_width: f32, new_height: f32) -> (f32, f32) {
    let aspect = width / height;
    let new_aspect = new_width / new_height;

    if new_aspect > aspect {
        (aspect / new_aspect, 1.0)
    } else {
        (1.0, new_aspect / aspect)
    }
}

pub(crate) fn fit(width: f32, height: f32, new_width: f32, new_height: f32) -> Affine2 {
    let (width_factor, height_factor) = factors(width, height, new_width, new_height);

    let transformation_matrix = Affine2::from_translation(Vec2::new(
        -1.0 + 1.0 - width_factor,
        1.0 - 1.0 + height_factor,
    )) * Affine2::from_scale(Vec2::new(
        2.0 / width * width_factor,
        -2.0 / height * height_factor,
    ));
    transformation_matrix
}
