// Main DOOM game

use crate::wad;

pub struct DoomGame {
    wad_data: wad::WadData,
}


impl DoomGame {
    pub fn new(wad_data: wad::WadData) -> DoomGame {
        DoomGame { wad_data }
    }

    // TODO to be continued ...

}
