pub struct ColorScheme {
    /*
    pub name: &'static str,

    // Color of the image border and of the cores of start pieces
    pub border_color: [u8; 3],

    pub background_color: [u8; 3],

    // Color of empty cells and the connecting lines between cells
    pub empty_cell_color: [u8; 3],
    */
    pub triangle_outer_color: [u8; 3],

    // Color of connecting pieces and the inner regions of start pieces
    pub triangle_color: [u8; 3],

    pub diamond_outer_color: [u8; 3],

    // Color of connecting pieces and the inner regions of start pieces
    pub diamond_color: [u8; 3],

    pub square_outer_color: [u8; 3],

    // Color of connecting pieces and the inner regions of start pieces
    pub square_color: [u8; 3],

    pub connector_color: [u8; 3],
}

pub fn determine_color_scheme(border_color: &[u8]) -> Option<&'static ColorScheme> {
    match border_color {
        [223, 232, 237] => Some(&KIND_OF_BLUE),
        [229, 221, 237] => Some(&SMOKING_ROOM),
        [232, 240, 222] => Some(&STONE_IN_FOCUS),
        [233, 239, 242] => Some(&MOONBASE),
        [233, 241, 223] => Some(&ORIGINAL),
        [235, 245, 247] => Some(&ELECTRO),
        [238, 224, 248] => Some(&TULIP),
        [239, 232, 223] => Some(&TANGERINE),
        [239, 234, 223] => Some(&DESERT),
        [239, 235, 223] => Some(&DEEP_SPACE),
        [242, 237, 247] => Some(&VELVET_ICE),
        [247, 243, 238] => Some(&PADDLEPOP),
        _ => None,
    }
}

const ORIGINAL: ColorScheme = ColorScheme {
    // name: "Original",
    // border_color: [233, 241, 223],
    // background_color: [121, 189, 154],
    // empty_cell_color: [137, 196, 164],
    triangle_outer_color: [206, 240, 183],
    triangle_color: [168, 219, 168],
    diamond_outer_color: [11, 72, 107],
    diamond_color: [59, 134, 134],
    square_outer_color: [190, 79, 35],
    square_color: [194, 120, 92],
    connector_color: [167, 219, 216],
};

const DEEP_SPACE: ColorScheme = ColorScheme {
    // name: "Deep Space",
    // border_color: [239, 235, 223],
    // background_color: [30, 30, 30],
    // empty_cell_color: [59, 59, 57],
    triangle_outer_color: [255, 165, 21],
    triangle_color: [206, 137, 48],
    diamond_outer_color: [0, 134, 255],
    diamond_color: [74, 178, 255],
    square_outer_color: [252, 31, 32],
    square_color: [198, 0, 0],
    connector_color: [209, 207, 184],
};

const MOONBASE: ColorScheme = ColorScheme {
    // name: "Moonbase",
    // border_color: [233, 239, 242],
    // background_color: [146, 148, 151],
    // empty_cell_color: [158, 161, 164],
    triangle_outer_color: [183, 94, 94],
    triangle_color: [170, 123, 123],
    diamond_outer_color: [82, 69, 86],
    diamond_color: [122, 117, 124],
    square_outer_color: [83, 116, 165],
    square_color: [115, 148, 175],
    connector_color: [190, 214, 142],
};

const STONE_IN_FOCUS: ColorScheme = ColorScheme {
    // name: "Stone in Focus",
    // border_color: [232, 240, 222],
    // background_color: [116, 149, 153],
    // empty_cell_color: [132, 162, 163],
    triangle_outer_color: [103, 103, 124],
    triangle_color: [186, 190, 204],
    diamond_outer_color: [175, 97, 44],
    diamond_color: [216, 138, 85],
    square_outer_color: [35, 31, 32],
    square_color: [88, 89, 91],
    connector_color: [186, 108, 123],
};

const VELVET_ICE: ColorScheme = ColorScheme {
    // name: "Velvet Ice",
    // border_color: [242, 237, 247],
    // background_color: [155, 97, 119],
    // empty_cell_color: [167, 117, 137],
    triangle_outer_color: [0, 192, 237],
    triangle_color: [61, 168, 196],
    diamond_outer_color: [153, 55, 55],
    diamond_color: [191, 64, 64],
    square_outer_color: [56, 56, 56],
    square_color: [20, 19, 19],
    connector_color: [214, 135, 131],
};

const ELECTRO: ColorScheme = ColorScheme {
    // name: "Electro",
    // border_color: [235, 245, 247],
    // background_color: [20, 20, 20],
    // empty_cell_color: [50, 51, 52],
    triangle_outer_color: [251, 0, 225],
    triangle_color: [216, 158, 212],
    diamond_outer_color: [81, 255, 0],
    diamond_color: [30, 214, 30],
    square_outer_color: [0, 249, 255],
    square_color: [112, 232, 228],
    connector_color: [251, 174, 23],
};

const TULIP: ColorScheme = ColorScheme {
    // name: "Tulip",
    // border_color: [238, 224, 248],
    // background_color: [151, 114, 177],
    // empty_cell_color: [164, 129, 187],
    triangle_outer_color: [155, 12, 87],
    triangle_color: [192, 81, 129],
    diamond_outer_color: [108, 159, 61],
    diamond_color: [149, 183, 137],
    square_outer_color: [15, 120, 151],
    square_color: [105, 151, 190],
    connector_color: [222, 168, 202],
};

const TANGERINE: ColorScheme = ColorScheme {
    // name: "Tangerine",
    // border_color: [239, 232, 223],
    // background_color: [211, 144, 110],
    // empty_cell_color: [215, 156, 126],
    triangle_outer_color: [181, 118, 163],
    triangle_color: [221, 151, 205],
    diamond_outer_color: [100, 107, 8],
    diamond_color: [147, 150, 29],
    square_outer_color: [40, 114, 188],
    square_color: [38, 169, 224],
    connector_color: [184, 200, 211],
};

const KIND_OF_BLUE: ColorScheme = ColorScheme {
    // name: "Kind of Blue",
    // border_color: [223, 232, 237],
    // background_color: [45, 162, 219],
    // empty_cell_color: [70, 172, 222],
    triangle_outer_color: [87, 200, 232],
    triangle_color: [157, 204, 215],
    diamond_outer_color: [164, 148, 85],
    diamond_color: [213, 191, 100],
    square_outer_color: [59, 75, 158],
    square_color: [78, 93, 169],
    connector_color: [167, 219, 215],
};

const SMOKING_ROOM: ColorScheme = ColorScheme {
    // name: "Smoking Room",
    // border_color: [229, 221, 237],
    // background_color: [82, 69, 86],
    // empty_cell_color: [103, 90, 107],
    triangle_outer_color: [254, 200, 23],
    triangle_color: [198, 162, 60],
    diamond_outer_color: [155, 102, 55],
    diamond_color: [135, 79, 41],
    square_outer_color: [120, 142, 158],
    square_color: [84, 100, 109],
    connector_color: [204, 185, 163],
};

const DESERT: ColorScheme = ColorScheme {
    // name: "Desert",
    // border_color: [239, 234, 223],
    // background_color: [196, 178, 148],
    // empty_cell_color: [202, 186, 158],
    triangle_outer_color: [239, 187, 101],
    triangle_color: [232, 159, 54],
    diamond_outer_color: [160, 78, 68],
    diamond_color: [181, 116, 111],
    square_outer_color: [27, 117, 187],
    square_color: [38, 169, 224],
    connector_color: [86, 86, 86],
};

const PADDLEPOP: ColorScheme = ColorScheme {
    // name: "Paddlepop",
    // border_color: [247, 243, 238],
    // background_color: [214, 185, 135],
    // empty_cell_color: [219, 193, 149],
    triangle_outer_color: [29, 219, 183],
    triangle_color: [111, 109, 188],
    diamond_outer_color: [244, 255, 31],
    diamond_color: [107, 249, 107],
    square_outer_color: [198, 23, 198],
    square_color: [214, 50, 136],
    connector_color: [188, 145, 81],
};
