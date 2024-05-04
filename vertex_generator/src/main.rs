/* Helper binary that copies a model by mirroring it on the x axis, and duplicating it down a
specified number of rows, outputting the list of vertices and indices at the end */

const MODEL_VERTICES: [f32; 54] = [

    // Front Pole
    -2.0, -0.6, 0.0, // bottom left - 0
    -1.95, -0.6, 0.0, // bottom right - 1
    -2.0, 1.2, 0.0,  // top left - 2
    -1.95, 1.2, 0.0, // top right - 3

    // Front Rim
    -2.0, 1.18, 0.0, // bottom left - 4
    -1.45, 1.18, 0.0, // bottom right - 5
    -2.0, 1.2, 0.0, // top left - 6
    -1.45, 1.2, 0.0, // top right - 7

    // Right rim
    -1.45, 1.18, -0.2, // bottom back - 8
    -1.45, 1.2, -0.2, // top back - 9

    // Back rim
    -2.0, 1.18, -0.2, //  - 10
    -2.0, 1.2, -0.2, // - 11

    // Left rim
    -2.0, 1.18, -0.0, //  - 12
    -2.0, 1.2, -0.0, // - 13


    // Back Pole
    -2.0, -0.6, -0.2, // bottom left - 14
    -1.95, -0.6, -0.2, // bottom right - 15
    -2.0, 1.2, -0.2,  // top left - 16
    -1.95, 1.2, -0.2, // top right - 17



    /*0.8, -0.6, 0.0, // bottom right - 4
    0.2, -0.6, 0.0, // bottom left - 5
    0.8, 0.0, 0.0, // top right
    0.2, 0.0, 0.0, // top left*/
];

const MODEL_INDICES: [i32; 36] = [
    0, 1, 2, 2, 1, 3, 

    4, 5, 6, 5, 6, 7,

    5, 7, 8, 7, 8, 9,

    8, 9, 10, 9, 10, 11,

    10, 11, 12, 11, 12, 13,

    14, 15, 16, 15, 16, 17
];


fn main() {
    let mut new_vertices: Vec<f32> = Vec::new();
    let mut new_indices: Vec<i32> = Vec::new();

    let distance_between: f32 = -3.0;
    let vertex_count: usize = MODEL_VERTICES.len() / 3;

    const TOTAL_ROWS: u8 = 10;

    // Generate rows
    for i in 0..TOTAL_ROWS {
        for (j_index, j) in MODEL_VERTICES.into_iter().enumerate() {
            let mut offset: f32 = 0.0;
            if (j_index+1)%3 == 0 { // Every third value (z value)
                offset = distance_between * i as f32;
            }
            new_vertices.push(j + offset);
        }

        for j in MODEL_INDICES{
            new_indices.push(j + (vertex_count as u8 * i) as i32);
        }
    }

    // Mirror on the X axis
    let vertex_count = new_vertices.len()/ 3;
    let temp_indices = new_indices.clone(); // Clone so length shenanigans doesnt fuck us over lel
    for i in temp_indices {
        new_indices.push(i + vertex_count as i32);
    }

    let temp_vertices = new_vertices.clone();
    for (i_index, i) in temp_vertices.into_iter().enumerate() {
        if (i_index+1)%3 == 1 { // Only negate x values
            new_vertices.push(i * -1.0);
        } else {
            new_vertices.push(i);
        }
    }

    println!("{:?}", new_vertices);
    println!("{:?}", new_indices);
}
