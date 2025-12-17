use linija::{
    edgemap::{EdgeMap, PathType},
    grid::Grid,
    image::{Image, png},
    tile::Tile,
};

fn main() {
    let files = std::env::args().skip(1).collect::<Vec<String>>();

    if files.is_empty() {
        panic!("No .png images specified on the command line");
    }

    for file in files {
        println!("Solving {}", file);
        println!();

        let image: Box<dyn Image> = png::open(&file).unwrap();

        let tiles = Tile::detect_tiles(&*image).unwrap();

        let grid = Grid::from_tiles(&tiles);

        let (edgemap, trail_triangle, trail_diamond, trail_square) =
            EdgeMap::find(&grid, |edgemap| {
                let trail_triangle = edgemap.trail(&grid, PathType::Triangle);
                let trail_diamond = edgemap.trail(&grid, PathType::Diamond);
                let trail_square = edgemap.trail(&grid, PathType::Square);

                if let Some(trail_triangle) = trail_triangle
                    && let Some(trail_diamond) = trail_diamond
                    && let Some(trail_square) = trail_square
                {
                    Err((edgemap.clone(), trail_triangle, trail_diamond, trail_square))
                } else {
                    Ok(())
                }
            })
            .unwrap_err();

        println!("{}", edgemap.printable(&grid));
        println!();

        for trail in [trail_triangle, trail_diamond, trail_square] {
            if !trail.is_empty() {
                println!(
                    "{}",
                    trail
                        .into_iter()
                        .map(|point| format!("{:?}", point))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
        }
        println!();

        println!("===");
        println!();
    }
}
