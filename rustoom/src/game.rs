// Main DOOM game

use crate::*;
use crate::map::*;
use crate::wad::*;
use crate::{GraphicsLoop, Painter};
use sdl2::event::Event;

const AUTOMAP_ZOOM_PERCENT: i32 = 10;

pub struct DoomGame {
    wad_data: WadData,
    scr_width: i32,
    scr_height: i32,
    _map_idx: usize,
    map: LevelMap,
}

impl DoomGame {
    pub fn new(wad_data: WadData, scr_width: i32, scr_height: i32) -> Result<DoomGame, String> {
        let first_map = wad_data.get_map(0)?;
        Ok(DoomGame {
            wad_data: wad_data,
            scr_width,
            scr_height,
            _map_idx: 0,
            map: first_map,
        })
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.wad_data.get_map_count()
    }

    pub fn load_map(&mut self, idx: usize) -> Result<(), String> {
        self._map_idx = idx;
        self.map = self.wad_data.get_map(idx)?;
        Ok(())
    }

    //----------------

    fn paint_automap(&self, painter: &mut dyn Painter) {
        painter.fill_rect(0, 0, self.scr_width, self.scr_height, BLACK);
        for line in self.map.line_defs.iter() {
            let v1 = self.translate_automap_vertex(line.v1_idx);
            let v2 = self.translate_automap_vertex(line.v2_idx);
            // TODO select color based on line type
            let color = YELLOW;
            painter.draw_line(v1.0, v1.1, v2.0, v2.1, color);
        }
    }

    #[inline]
    fn translate_automap_vertex(&self, vertex_idx: u16) -> (i32, i32) {
        let orig_vertex = self.map.get_vertex(vertex_idx);
        // TODO variable translation + scaling !!
        let x = ((orig_vertex.x - self.map.x_min) as i32) * AUTOMAP_ZOOM_PERCENT / 100;
        let yf = ((orig_vertex.y - self.map.y_min) as i32) * AUTOMAP_ZOOM_PERCENT / 100;
        let y = self.scr_height - yf - 1;
        (x, y)
    }
}

impl GraphicsLoop for DoomGame {
    fn handle_event(&mut self, _event: &Event) -> bool {
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        self.paint_automap(painter);
    }
}
