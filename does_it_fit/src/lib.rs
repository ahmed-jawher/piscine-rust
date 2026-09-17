pub mod areas_volumes;
pub use crate::areas_volumes::*;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rec_area = x * y;
    let shape_area = match kind {
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    if rec_area < (shape_area * times as f64) as usize {
        return false;
    }

    let shape_across_x = match kind {
        GeometricalShapes::Circle => x / (2 * a),
        GeometricalShapes::Rectangle => x / a,
        GeometricalShapes::Square => x / a,
        GeometricalShapes::Triangle => x / a,
    };

    let shape_across_y = match kind {
        GeometricalShapes::Circle => y / (2 * a),
        GeometricalShapes::Rectangle => y / b,
        GeometricalShapes::Square => y / a,
        GeometricalShapes::Triangle => y / b,
    };

    let total_shapes = shape_across_x * shape_across_y;

    if total_shapes > times && times != 1 {
        return false;
    }

    true
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let rec_volume = x * y * z;

    let shape_volume = match kind {
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
    };

    if rec_volume < (shape_volume * times as f64) as usize {
        return false;
    }

    let shape_across_x = match kind {
        GeometricalVolumes::Cone => x / a,
        GeometricalVolumes::Cube => x / a,
        GeometricalVolumes::Parallelepiped => x / a,
        GeometricalVolumes::Sphere => x / (2 * a),
        GeometricalVolumes::TriangularPyramid => x / a,
    };

    let shape_across_y = match kind {
        GeometricalVolumes::Cone => y / b,
        GeometricalVolumes::Cube => y / a,
        GeometricalVolumes::Parallelepiped => y / b,
        GeometricalVolumes::Sphere => y / (2 * a),
        GeometricalVolumes::TriangularPyramid => y / b,
    };

    let shape_across_z = match kind {
        GeometricalVolumes::Cone => z / b,
        GeometricalVolumes::Cube => z / a,
        GeometricalVolumes::Parallelepiped => z / c,
        GeometricalVolumes::Sphere => z / (2 * a),
        GeometricalVolumes::TriangularPyramid => z / c,
    };

    let space_shape_takes = shape_across_x * shape_across_y * shape_across_z;

    if space_shape_takes < times && times > 1 {
        return false;
    }

    true
}
