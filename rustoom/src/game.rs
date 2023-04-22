// Main DOOM game

use crate::wad;

pub struct DoomGame<'a> {
    wad_data: wad::WadData<'a>,
}


impl<'a> DoomGame<'a> {
    pub fn new(wad_data: wad::WadData) -> DoomGame {
        DoomGame { wad_data }
    }

    // TODO to be continued ...

}
