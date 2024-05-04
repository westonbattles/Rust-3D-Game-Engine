use cgmath::{perspective, Deg, Matrix4, Point3, Rad, Vector3};
use std::ops::Add;
use std::{mem, ptr};
use std::time::{Duration, Instant};
use rge::graphics::{gl_wrapper::{BufferObject, ShaderProgram, SHADER_PROGRAM, Vao, VertexAttribute}, window::Window};

const VERTICES: [f32; 1092] = 
[-2.0, -0.6, 0.0, -1.95, -0.6, 0.0, -2.0, 1.2, 0.0, -1.95, 1.2, 0.0, -2.0, 1.18, 0.0, -1.45, 1.18, 0.0, -2.0, 1.2, 0.0, -1.45, 1.2, 0.0, -1.45, 1.18, -0.2, -1.45, 1.2, -0.2, -2.0, 1.18, -0.2, -2.0, 1.2, -0.2, -2.0, 1.18, -0.0, -2.0, 1.2, -0.0, -2.0, -0.6, -0.2, -1.95, -0.6, -0.2, -2.0, 1.2, -0.2, -1.95, 1.2, -0.2, -2.0, -0.6, -3.0, -1.95, -0.6, -3.0, -2.0, 1.2, -3.0, -1.95, 1.2, -3.0, -2.0, 1.18, -3.0, -1.45, 1.18, -3.0, -2.0, 1.2, -3.0, -1.45, 1.2, -3.0, -1.45, 1.18, -3.2, -1.45, 1.2, -3.2, -2.0, 1.18, -3.2, -2.0, 1.2, -3.2, -2.0, 1.18, -3.0, -2.0, 1.2, -3.0, -2.0, -0.6, -3.2, -1.95, -0.6, -3.2, -2.0, 1.2, -3.2, -1.95, 1.2, -3.2, -2.0, -0.6, -6.0, -1.95, -0.6, -6.0, -2.0, 1.2, -6.0, -1.95, 1.2, -6.0, -2.0, 1.18, -6.0, -1.45, 1.18, -6.0, -2.0, 1.2, -6.0, -1.45, 1.2, -6.0, -1.45, 1.18, -6.2, -1.45, 1.2, -6.2, -2.0, 1.18, -6.2, -2.0, 1.2, -6.2, -2.0, 1.18, -6.0, -2.0, 1.2, -6.0, -2.0, -0.6, -6.2, -1.95, -0.6, -6.2, -2.0, 1.2, -6.2, -1.95, 1.2, -6.2, -2.0, -0.6, -9.0, -1.95, -0.6, -9.0, -2.0, 1.2, -9.0, -1.95, 1.2, -9.0, -2.0, 1.18, -9.0, -1.45, 1.18, -9.0, -2.0, 1.2, -9.0, -1.45, 1.2, -9.0, -1.45, 1.18, -9.2, -1.45, 1.2, -9.2, -2.0, 1.18, -9.2, -2.0, 1.2, -9.2, -2.0, 1.18, -9.0, -2.0, 1.2, -9.0, -2.0, -0.6, -9.2, -1.95, -0.6, -9.2, -2.0, 1.2, -9.2, -1.95, 1.2, -9.2, -2.0, -0.6, -12.0, -1.95, -0.6, -12.0, -2.0, 1.2, -12.0, -1.95, 1.2, -12.0, -2.0, 1.18, -12.0, -1.45, 1.18, -12.0, -2.0, 1.2, -12.0, -1.45, 1.2, -12.0, -1.45, 1.18, -12.2, -1.45, 1.2, -12.2, -2.0, 1.18, -12.2, -2.0, 1.2, -12.2, -2.0, 1.18, -12.0, -2.0, 1.2, -12.0, -2.0, -0.6, -12.2, -1.95, -0.6, -12.2, -2.0, 1.2, -12.2, -1.95, 1.2, -12.2, -2.0, -0.6, -15.0, -1.95, -0.6, -15.0, -2.0, 1.2, -15.0, -1.95, 1.2, -15.0, -2.0, 1.18, -15.0, -1.45, 1.18, -15.0, -2.0, 1.2, -15.0, -1.45, 1.2, -15.0, -1.45, 1.18, -15.2, -1.45, 1.2, -15.2, -2.0, 1.18, -15.2, -2.0, 1.2, -15.2, -2.0, 1.18, -15.0, -2.0, 1.2, -15.0, -2.0, -0.6, -15.2, -1.95, -0.6, -15.2, -2.0, 1.2, -15.2, -1.95, 1.2, -15.2, -2.0, -0.6, -18.0, -1.95, -0.6, -18.0, -2.0, 1.2, -18.0, -1.95, 1.2, -18.0, -2.0, 1.18, -18.0, -1.45, 1.18, -18.0, -2.0, 1.2, -18.0, -1.45, 1.2, -18.0, -1.45, 1.18, -18.2, -1.45, 1.2, -18.2, -2.0, 1.18, -18.2, -2.0, 1.2, -18.2, -2.0, 1.18, -18.0, -2.0, 1.2, -18.0, -2.0, -0.6, -18.2, -1.95, -0.6, -18.2, -2.0, 1.2, -18.2, -1.95, 1.2, -18.2, -2.0, -0.6, -21.0, -1.95, -0.6, -21.0, -2.0, 1.2, -21.0, -1.95, 1.2, -21.0, -2.0, 1.18, -21.0, -1.45, 1.18, -21.0, -2.0, 1.2, -21.0, -1.45, 1.2, -21.0, -1.45, 1.18, -21.2, -1.45, 1.2, -21.2, -2.0, 1.18, -21.2, -2.0, 1.2, -21.2, -2.0, 1.18, -21.0, -2.0, 1.2, -21.0, -2.0, -0.6, -21.2, -1.95, -0.6, -21.2, -2.0, 1.2, -21.2, -1.95, 1.2, -21.2, -2.0, -0.6, -24.0, -1.95, -0.6, -24.0, -2.0, 1.2, -24.0, -1.95, 1.2, -24.0, -2.0, 1.18, -24.0, -1.45, 1.18, -24.0, -2.0, 1.2, -24.0, -1.45, 1.2, -24.0, -1.45, 1.18, -24.2, -1.45, 1.2, -24.2, -2.0, 1.18, -24.2, -2.0, 1.2, -24.2, -2.0, 1.18, -24.0, -2.0, 1.2, -24.0, -2.0, -0.6, -24.2, -1.95, -0.6, -24.2, -2.0, 1.2, -24.2, -1.95, 1.2, -24.2, -2.0, -0.6, -27.0, -1.95, -0.6, -27.0, -2.0, 1.2, -27.0, -1.95, 1.2, -27.0, -2.0, 1.18, -27.0, -1.45, 1.18, -27.0, -2.0, 1.2, -27.0, -1.45, 1.2, -27.0, -1.45, 1.18, -27.2, -1.45, 1.2, -27.2, -2.0, 1.18, -27.2, -2.0, 1.2, -27.2, -2.0, 1.18, -27.0, -2.0, 1.2, -27.0, -2.0, -0.6, -27.2, -1.95, -0.6, -27.2, -2.0, 1.2, -27.2, -1.95, 1.2, -27.2, 2.0, -0.6, 0.0, 1.95, -0.6, 0.0, 2.0, 1.2, 0.0, 1.95, 1.2, 0.0, 2.0, 1.18, 0.0, 1.45, 1.18, 0.0, 2.0, 1.2, 0.0, 1.45, 1.2, 0.0, 1.45, 1.18, -0.2, 1.45, 1.2, -0.2, 2.0, 1.18, -0.2, 2.0, 1.2, -0.2, 2.0, 1.18, -0.0, 2.0, 1.2, -0.0, 2.0, -0.6, -0.2, 1.95, -0.6, -0.2, 2.0, 1.2, -0.2, 1.95, 1.2, -0.2, 2.0, -0.6, -3.0, 1.95, -0.6, -3.0, 2.0, 1.2, -3.0, 1.95, 1.2, -3.0, 2.0, 1.18, -3.0, 1.45, 1.18, -3.0, 2.0, 1.2, -3.0, 1.45, 1.2, -3.0, 1.45, 1.18, -3.2, 1.45, 1.2, -3.2, 2.0, 1.18, -3.2, 2.0, 1.2, -3.2, 2.0, 1.18, -3.0, 2.0, 1.2, -3.0, 2.0, -0.6, -3.2, 1.95, -0.6, -3.2, 2.0, 1.2, -3.2, 1.95, 1.2, -3.2, 2.0, -0.6, -6.0, 1.95, -0.6, -6.0, 2.0, 1.2, -6.0, 1.95, 1.2, -6.0, 2.0, 1.18, -6.0, 1.45, 1.18, -6.0, 2.0, 1.2, -6.0, 1.45, 1.2, -6.0, 1.45, 1.18, -6.2, 1.45, 1.2, -6.2, 2.0, 1.18, -6.2, 2.0, 1.2, -6.2, 2.0, 1.18, -6.0, 2.0, 1.2, -6.0, 2.0, -0.6, -6.2, 1.95, -0.6, -6.2, 2.0, 1.2, -6.2, 1.95, 1.2, -6.2, 2.0, -0.6, -9.0, 1.95, -0.6, -9.0, 2.0, 1.2, -9.0, 1.95, 1.2, -9.0, 2.0, 1.18, -9.0, 1.45, 1.18, -9.0, 2.0, 1.2, -9.0, 1.45, 1.2, -9.0, 1.45, 1.18, -9.2, 1.45, 1.2, -9.2, 2.0, 1.18, -9.2, 2.0, 1.2, -9.2, 2.0, 1.18, -9.0, 2.0, 1.2, -9.0, 2.0, -0.6, -9.2, 1.95, -0.6, -9.2, 2.0, 1.2, -9.2, 1.95, 1.2, -9.2, 2.0, -0.6, -12.0, 1.95, -0.6, -12.0, 2.0, 1.2, -12.0, 1.95, 1.2, -12.0, 2.0, 1.18, -12.0, 1.45, 1.18, -12.0, 2.0, 1.2, -12.0, 1.45, 1.2, -12.0, 1.45, 1.18, -12.2, 1.45, 1.2, -12.2, 2.0, 1.18, -12.2, 2.0, 1.2, -12.2, 2.0, 1.18, -12.0, 2.0, 1.2, -12.0, 2.0, -0.6, -12.2, 1.95, -0.6, -12.2, 2.0, 1.2, -12.2, 1.95, 1.2, -12.2, 2.0, -0.6, -15.0, 1.95, -0.6, -15.0, 2.0, 1.2, -15.0, 1.95, 1.2, -15.0, 2.0, 1.18, -15.0, 1.45, 1.18, -15.0, 2.0, 1.2, -15.0, 1.45, 1.2, -15.0, 1.45, 1.18, -15.2, 1.45, 1.2, -15.2, 2.0, 1.18, -15.2, 2.0, 1.2, -15.2, 2.0, 1.18, -15.0, 2.0, 1.2, -15.0, 2.0, -0.6, -15.2, 1.95, -0.6, -15.2, 2.0, 1.2, -15.2, 1.95, 1.2, -15.2, 2.0, -0.6, -18.0, 1.95, -0.6, -18.0, 2.0, 1.2, -18.0, 1.95, 1.2, -18.0, 2.0, 1.18, -18.0, 1.45, 1.18, -18.0, 2.0, 1.2, -18.0, 1.45, 1.2, -18.0, 1.45, 1.18, -18.2, 1.45, 1.2, -18.2, 2.0, 1.18, -18.2, 2.0, 1.2, -18.2, 2.0, 1.18, -18.0, 2.0, 1.2, -18.0, 2.0, -0.6, -18.2, 1.95, -0.6, -18.2, 2.0, 1.2, -18.2, 1.95, 1.2, -18.2, 2.0, -0.6, -21.0, 1.95, -0.6, -21.0, 2.0, 1.2, -21.0, 1.95, 1.2, -21.0, 2.0, 1.18, -21.0, 1.45, 1.18, -21.0, 2.0, 1.2, -21.0, 1.45, 1.2, -21.0, 1.45, 1.18, -21.2, 1.45, 1.2, -21.2, 2.0, 1.18, -21.2, 2.0, 1.2, -21.2, 2.0, 1.18, -21.0, 2.0, 1.2, -21.0, 2.0, -0.6, -21.2, 1.95, -0.6, -21.2, 2.0, 1.2, -21.2, 1.95, 1.2, -21.2, 2.0, -0.6, -24.0, 1.95, -0.6, -24.0, 2.0, 1.2, -24.0, 1.95, 1.2, -24.0, 2.0, 1.18, -24.0, 1.45, 1.18, -24.0, 2.0, 1.2, -24.0, 1.45, 1.2, -24.0, 1.45, 1.18, -24.2, 1.45, 1.2, -24.2, 2.0, 1.18, -24.2, 2.0, 1.2, -24.2, 2.0, 1.18, -24.0, 2.0, 1.2, -24.0, 2.0, -0.6, -24.2, 1.95, -0.6, -24.2, 2.0, 1.2, -24.2, 1.95, 1.2, -24.2, 2.0, -0.6, -27.0, 1.95, -0.6, -27.0, 2.0, 1.2, -27.0, 1.95, 1.2, -27.0, 2.0, 1.18, -27.0, 1.45, 1.18, -27.0, 2.0, 1.2, -27.0, 1.45, 1.2, -27.0, 1.45, 1.18, -27.2, 1.45, 1.2, -27.2, 2.0, 1.18, -27.2, 2.0, 1.2, -27.2, 2.0, 1.18, -27.0, 2.0, 1.2, -27.0, 2.0, -0.6, -27.2, 1.95, -0.6, -27.2, 2.0, 1.2, -27.2, 1.95, 1.2, -27.2

// ROAD
,-1.5, -0.6, 0.0,
1.5, -0.6, 0.0,
-1.5, -0.6, -27.2,
1.5, -0.6, -27.2,

];

const INDICES: [i32; 726] =
[0, 1, 2, 2, 1, 3, 4, 5, 6, 5, 6, 7, 5, 7, 8, 7, 8, 9, 8, 9, 10, 9, 10, 11, 10, 11, 12, 11, 12, 13, 14, 15, 16, 15, 16, 17, 18, 19, 20, 20, 19, 21, 22, 23, 24, 23, 24, 25, 23, 25, 26, 25, 26, 27, 26, 27, 28, 27, 28, 29, 28, 29, 30, 29, 30, 31, 32, 33, 34, 33, 34, 35, 36, 37, 38, 38, 37, 39, 40, 41, 42, 41, 42, 43, 41, 43, 44, 43, 44, 45, 44, 45, 46, 45, 46, 47, 46, 47, 48, 47, 48, 49, 50, 51, 52, 51, 52, 53, 54, 55, 56, 56, 55, 57, 58, 59, 60, 59, 60, 61, 59, 61, 62, 61, 62, 63, 62, 63, 64, 63, 64, 65, 64, 65, 66, 65, 66, 67, 68, 69, 70, 69, 70, 71, 72, 73, 74, 74, 73, 75, 76, 77, 78, 77, 78, 79, 77, 79, 80, 79, 80, 81, 80, 81, 82, 81, 82, 83, 82, 83, 84, 83, 84, 85, 86, 87, 88, 87, 88, 89, 90, 91, 92, 92, 91, 93, 94, 95, 96, 95, 96, 97, 95, 97, 98, 97, 98, 99, 98, 99, 100, 99, 100, 101, 100, 101, 102, 101, 102, 103, 104, 105, 106, 105, 106, 107, 108, 109, 110, 110, 109, 111, 112, 113, 114, 113, 114, 115, 113, 115, 116, 115, 116, 117, 116, 117, 118, 117, 118, 119, 118, 119, 120, 119, 120, 121, 122, 123, 124, 123, 124, 125, 126, 127, 128, 128, 127, 129, 130, 131, 132, 131, 132, 133, 131, 133, 134, 133, 134, 135, 134, 135, 136, 135, 136, 137, 136, 137, 138, 137, 138, 139, 140, 141, 142, 141, 142, 143, 144, 145, 146, 146, 145, 147, 148, 149, 150, 149, 150, 151, 149, 151, 152, 151, 152, 153, 152, 153, 154, 153, 154, 155, 154, 155, 156, 155, 156, 157, 158, 159, 160, 159, 160, 161, 162, 163, 164, 164, 163, 165, 166, 167, 168, 167, 168, 169, 167, 169, 170, 169, 170, 171, 170, 171, 172, 171, 172, 173, 172, 173, 174, 173, 174, 175, 176, 177, 178, 177, 178, 179, 180, 181, 182, 182, 181, 183, 184, 185, 186, 185, 186, 187, 185, 187, 188, 187, 188, 189, 188, 189, 190, 189, 190, 191, 190, 191, 192, 191, 192, 193, 194, 195, 196, 195, 196, 197, 198, 199, 200, 200, 199, 201, 202, 203, 204, 203, 204, 205, 203, 205, 206, 205, 206, 207, 206, 207, 208, 207, 208, 209, 208, 209, 210, 209, 210, 211, 212, 213, 214, 213, 214, 215, 216, 217, 218, 218, 217, 219, 220, 221, 222, 221, 222, 223, 221, 223, 224, 223, 224, 225, 224, 225, 226, 225, 226, 227, 226, 227, 228, 227, 228, 229, 230, 231, 232, 231, 232, 233, 234, 235, 236, 236, 235, 237, 238, 239, 240, 239, 240, 241, 239, 241, 242, 241, 242, 243, 242, 243, 244, 243, 244, 245, 244, 245, 246, 245, 246, 247, 248, 249, 250, 249, 250, 251, 252, 253, 254, 254, 253, 255, 256, 257, 258, 257, 258, 259, 257, 259, 260, 259, 260, 261, 260, 261, 262, 261, 262, 263, 262, 263, 264, 263, 264, 265, 266, 267, 268, 267, 268, 269, 270, 271, 272, 272, 271, 273, 274, 275, 276, 275, 276, 277, 275, 277, 278, 277, 278, 279, 278, 279, 280, 279, 280, 281, 280, 281, 282, 281, 282, 283, 284, 285, 286, 285, 286, 287, 288, 289, 290, 290, 289, 291, 292, 293, 294, 293, 294, 295, 293, 295, 296, 295, 296, 297, 296, 297, 298, 297, 298, 299, 298, 299, 300, 299, 300, 301, 302, 303, 304, 303, 304, 305, 306, 307, 308, 308, 307, 309, 310, 311, 312, 311, 312, 313, 311, 313, 314, 313, 314, 315, 314, 315, 316, 315, 316, 317, 316, 317, 318, 317, 318, 319, 320, 321, 322, 321, 322, 323, 324, 325, 326, 326, 325, 327, 328, 329, 330, 329, 330, 331, 329, 331, 332, 331, 332, 333, 332, 333, 334, 333, 334, 335, 334, 335, 336, 335, 336, 337, 338, 339, 340, 339, 340, 341, 342, 343, 344, 344, 343, 345, 346, 347, 348, 347, 348, 349, 347, 349, 350, 349, 350, 351, 350, 351, 352, 351, 352, 353, 352, 353, 354, 353, 354, 355, 356, 357, 358, 357, 358, 359, 

360, 361, 362, 361, 362, 363];
/* 
const VERTICES: [f32; 54] = [

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

const INDICES: [i32; 36] = [
    0, 1, 2, 2, 1, 3, 

    4, 5, 6, 5, 6, 7,

    5, 7, 8, 7, 8, 9,

    8, 9, 10, 9, 10, 11,

    10, 11, 12, 11, 12, 13,

    14, 15, 16, 15, 16, 17
];*/


/* 
const VERTICES: [f32; 24] = [
    // Back face
    -0.5, -0.5, -0.5, 
     0.5, -0.5, -0.5, 
     0.5,  0.5, -0.5, 
    -0.5,  0.5, -0.5, 

    // Front face
    -0.5, -0.5,  0.5, 
     0.5, -0.5,  0.5, 
     0.5,  0.5,  0.5, 
    -0.5,  0.5,  0.5, 
];

const INDICES: [u32; 36] = [
    // Back face
    0, 1, 2, 2, 3, 0,
    // Front face
    4, 5, 6, 6, 7, 4,
    // Left face
    0, 4, 7, 7, 3, 0,
    // Right face
    1, 5, 6, 6, 2, 1,
    // Bottom face
    0, 1, 5, 5, 4, 0,
    // Top face
    3, 2, 6, 6, 7, 3
];*/

fn main() {
    let mut window = Window::new(1080, 720, "WINDOW WOWOWOWO");
    window.init_gl();


    // Vertex array object
    let vao = Vao::new();
    vao.bind();

    // Vertex buffer object
    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_data(&VERTICES);

    // Index buffer object
    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();
    ibo.store_data(&INDICES);

    let position_attribute = VertexAttribute::new(
        0, 
        3, 
        gl::FLOAT, 
        gl::FALSE, 
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );

    let index_attribute = VertexAttribute::new(
        1, 
        3, 
        gl::FLOAT, 
        gl::FALSE, 
        3 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
        ptr::null(),
    );

    position_attribute.enable();
    index_attribute.enable();


    let projection: Matrix4<f32> = perspective(Deg(45.0), window.get_aspect_ratio(), 0.1, 100.0);
    let mut camera_pos: Point3<f32> = Point3::new(0.0, 0.0, 3.0); // Camera position
    let camera_facing: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0); // Where the camera is looking
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0); // Up direction
    let mut view: Matrix4<f32> = Matrix4::look_at_rh(camera_pos, camera_pos.add(camera_facing), up);
    //let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0));
    let mut model: Matrix4<f32>; //Matrix4::from_angle_x(Deg(-55.0));
    {
        let mut shader_program_gaurd = SHADER_PROGRAM.lock().unwrap();
        *shader_program_gaurd = Some(ShaderProgram::new(
            "./rge/src/graphics/shaders/vertex_shader.glsl", 
            "./rge/src/graphics/shaders/fragment_shader.glsl"
        ));

        if let Some(shader_program) = shader_program_gaurd.as_mut() {
            shader_program.bind();
            shader_program.load_uniform("aspectRatio");
            //shader_program.load_uniform("translation");
            //shader_program.load_uniform("rotationMatrix");
            shader_program.load_uniform("projection");
            shader_program.load_uniform("view");
            shader_program.load_uniform("model");
            shader_program.set_float_uniform("aspectRatio", window.get_aspect_ratio());
            shader_program.set_matrix4fv_uniform("projection", &projection);
            shader_program.set_matrix4fv_uniform("view", &view);
        }
        
    }
    
    window.set_framebuffer_size_callback();

    // MAIN GAME LOOP


    let mut last_frame_time = Instant::now();
    let mut delta_time: Duration;

    let mut current_angle = 0.0;

    while !window.should_close() {

        let current_frame_time = Instant::now();
        delta_time = current_frame_time-last_frame_time;
        last_frame_time = current_frame_time;



        //rotation and transformation
        current_angle += window.input_array[0] as f32 * 100.0 * delta_time.as_secs_f32();

        let angle = Rad(current_angle.to_radians());
        let forward_offset = (current_angle-90.0).to_radians(); // Offset to make forward up initially
        let direction: Vector3<f32> = Vector3::new(forward_offset.cos(), 0.0, forward_offset.sin());

        camera_pos += window.input_array[1] as f32 * 1.0 * delta_time.as_secs_f32() * direction;


        //model = Matrix4::from_axis_angle(Vector3::new(0.5, 1.0, 1.0), Deg(50.0 + (window.glfw.get_time() * 100.0) as f32));
        model = Matrix4::from_translation(Vector3::new(camera_pos.x, camera_pos.y, camera_pos.z)) * Matrix4::from_angle_y(angle) * Matrix4::from_translation(Vector3::new(-camera_pos.x, -camera_pos.y, -camera_pos.z));
        view = Matrix4::look_at_rh(camera_pos, camera_pos.add(camera_facing), up);


        {
            let mut shader_program_gaurd = SHADER_PROGRAM.lock().unwrap();
            if let Some(shader_program) = shader_program_gaurd.as_mut() {
                //shader_program.set_vec3_uniform("translation", transpose_vector);
                //shader_program.set_matrix4fv_uniform("rotationMatrix", &rot_matrix);
                shader_program.set_matrix4fv_uniform("view", &view);
                shader_program.set_matrix4fv_uniform("model", &model);

            }
        }

        rge::graphics::drawer::draw();
        window.update();
    }
}
/*
use rge::graphics::window::Window;

const BACKGROUND_COLOR: (f32, f32, f32, f32) = (0.4, 0.6, 0.4, 1.0);

fn window_test() {
    let mut window = Window::new(1080, 720, "WINDOW WOWOWOWO");

    window.init_gl();

    while !window.should_close() {
        unsafe {

            // Updates the window with the background color 
            let (r,g,b,a) = BACKGROUND_COLOR;
            gl::ClearColor(r,g,b,a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.update();
    }
}
*/

/* 
use rge::custom_errors::Errors;

fn error_test(num: i32) -> Result<(), Errors> {
    if num == 1 {
        rge::logger::info!("Error");
        return Err(Errors::TestError.into());
    }
    Ok(())
}
 */
