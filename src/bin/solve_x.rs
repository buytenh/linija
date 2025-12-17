use std::{thread::sleep, time::Duration};

use linija::{
    edgemap::{EdgeMap, PathType},
    grid::Grid,
    image,
    tile::Tile,
};
use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{AtomEnum, BUTTON_PRESS_EVENT, BUTTON_RELEASE_EVENT, ConnectionExt},
        xtest::ConnectionExt as XtestConnectionExt,
    },
    rust_connection::RustConnection,
};

fn main() {
    let mut conn = Conn::new(None);

    let window_name = "LYNE";

    let Some(window) = conn.find_window_by_name(window_name) else {
        panic!("Can't find X window '{}'", window_name);
    };

    let image = image::x::from_window(&conn.conn, window).unwrap();

    let tiles = Tile::detect_tiles(&*image).unwrap();

    let grid = Grid::from_tiles(&tiles);

    let (edgemap, trail_triangle, trail_diamond, trail_square) = EdgeMap::find(&grid, |edgemap| {
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

    let max_pixels_per_second = 650;
    let hz: u16 = 20;

    let max_pixels_per_time_unit: i32 = max_pixels_per_second / i32::from(hz);
    let time_unit: Duration = Duration::from_millis(1000 / u64::from(hz));

    for trail in [trail_triangle, trail_diamond, trail_square] {
        if !trail.is_empty() {
            println!();

            for (i, point) in trail.iter().enumerate() {
                println!(
                    "{:?}    {:?}",
                    point,
                    (grid.column_x[point.1], grid.row_y[point.0])
                );

                conn.move_to(
                    window,
                    grid.column_x[point.1],
                    grid.row_y[point.0],
                    max_pixels_per_time_unit,
                    time_unit,
                );

                if i == 0 {
                    conn.button_down(window);
                    sleep(time_unit);
                }
            }

            conn.button_up(window);
            sleep(time_unit);
        }
    }
}

struct Conn {
    conn: RustConnection,
    screen_num: usize,
    position: Option<(u16, u16)>,
}

impl Conn {
    fn new(dpy_name: Option<&str>) -> Self {
        let (conn, screen_num) = x11rb::connect(dpy_name).unwrap();

        Conn {
            conn,
            screen_num,
            position: None,
        }
    }

    fn find_window_by_name(&self, name: &str) -> Option<u32> {
        self.find_window_by_name_in_tree(self.conn.setup().roots[self.screen_num].root, 0, name)
    }

    fn find_window_by_name_in_tree(&self, root: u32, depth: usize, name: &str) -> Option<u32> {
        let wm_name = self
            .conn
            .get_property(false, root, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 0x1000)
            .unwrap()
            .reply()
            .unwrap();

        if false {
            println!(
                "{}{:08x}: {}",
                "    ".repeat(depth),
                root,
                str::from_utf8(&wm_name.value).unwrap()
            );
        }

        // Check children before checking the parent, so that we return the most deeply
        // nested window that has the name we are looking for.
        for child in self
            .conn
            .query_tree(root)
            .unwrap()
            .reply()
            .unwrap()
            .children
        {
            if let Some(window) = self.find_window_by_name_in_tree(child, depth + 1, name) {
                return Some(window);
            }
        }

        if wm_name.value == name.as_bytes() {
            return Some(root);
        }

        None
    }

    fn move_to(
        &mut self,
        window: u32,
        to_x: u16,
        to_y: u16,
        max_pixels_per_time_unit: i32,
        time_unit: Duration,
    ) {
        while self.position != Some((to_x, to_y)) {
            let (new_x, new_y) = match self.position {
                None => (to_x, to_y),
                Some((from_x, from_y)) => {
                    let diff_x = i32::from(to_x) - i32::from(from_x);
                    let diff_y = i32::from(to_y) - i32::from(from_y);

                    let distance_sq = diff_x * diff_x + diff_y * diff_y;

                    if distance_sq <= max_pixels_per_time_unit * max_pixels_per_time_unit {
                        (to_x, to_y)
                    } else {
                        let distance = (distance_sq as f64).sqrt() as i32;

                        let move_x = (diff_x * max_pixels_per_time_unit) / distance;
                        let move_y = (diff_y * max_pixels_per_time_unit) / distance;

                        let to_x = u16::try_from(i32::from(from_x) + move_x).unwrap();
                        let to_y = u16::try_from(i32::from(from_y) + move_y).unwrap();

                        (to_x, to_y)
                    }
                }
            };

            self.warp_pointer(window, new_x, new_y);

            self.position = Some((new_x, new_y));

            sleep(time_unit);
        }
    }

    fn warp_pointer(&self, window: u32, x: u16, y: u16) {
        self.conn
            .warp_pointer(
                0u32,
                window,
                0,
                0,
                0,
                0,
                i16::try_from(x).unwrap(),
                i16::try_from(y).unwrap(),
            )
            .unwrap();

        self.conn.flush().unwrap();
    }

    fn button_down(&self, window: u32) {
        self.conn
            .xtest_fake_input(BUTTON_PRESS_EVENT, 1, 0, window, 0, 0, 0)
            .unwrap();

        self.conn.flush().unwrap();
    }

    fn button_up(&self, window: u32) {
        self.conn
            .xtest_fake_input(BUTTON_RELEASE_EVENT, 1, 0, window, 0, 0, 0)
            .unwrap();

        self.conn.flush().unwrap();
    }
}
