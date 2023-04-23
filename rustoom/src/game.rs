// Main DOOM game

use crate::wad;

pub struct DoomGame {
    wad_data: wad::WadData,
    // TODO to be continued ...
}


impl DoomGame {
    pub fn new(wad_data: wad::WadData) -> Result<DoomGame, String> {
        Self::validate_wad_data(&wad_data)?;
        Ok(DoomGame { wad_data })
    }

    fn validate_wad_data(wad_data: &wad::WadData) -> Result<(), String> {
        // check that all lumps are ok
        let lump_count = wad_data.get_lump_count();
        if lump_count == 0 {
            return Err(String::from("WAD has no lumps"));
        }
        // TODO - TEMP logging
        println!("[DBG] WAD Lump Count: {lump_count}");
        for i in 0 .. lump_count {
            let lump = wad_data.get_lump(i)?;
            println!("[DBG]   => {:4}: {:8} -> len={}", i, lump.name, lump.bytes.len());
        }

        // TODO to be continued ...

        Ok(())
    }

    // TODO to be continued ...

}
