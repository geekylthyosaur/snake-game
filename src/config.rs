pub struct Config {
    pub map_width_px: usize,
    pub map_height_px: usize,
    pub cell_width_px: usize,
    pub cell_height_px: usize,
}

impl Config {
    pub fn new(
        map_width_px: usize,
        map_height_px: usize,
        cell_width_px: usize,
        cell_height_px: usize,
    ) -> Self {
        Self {
            map_width_px,
            map_height_px,
            cell_width_px,
            cell_height_px,
        }
    }
}
