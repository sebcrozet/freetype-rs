
use std::num::FromPrimitive;
use ffi::*;
use FtResult;

pub struct GlyphSlot {
    raw: FT_GlyphSlot,
}

impl GlyphSlot {
    pub fn new(raw: FT_GlyphSlot) -> GlyphSlot {
        GlyphSlot {
            raw: raw,
        }
    }

    pub fn render_glyph(&self, render_mode: FT_Render_Mode) -> FtResult<()> {
        unsafe {
            let err = FT_Render_Glyph(self.raw(), render_mode);
            if err == 0 {
                Ok(())
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    pub fn get_subglyph_info(&self, sub_index: FT_UInt) -> FtResult<(FT_Int, FT_UInt, FT_Int, FT_Int, FT_Matrix)> {
        unsafe {
            let index = 0;
            let flags = 0;
            let arg1 = 0;
            let arg2 = 0;
            let transfrom = FT_Matrix {
                xx: 0, xy: 0,
                yx: 0, yy: 0,
            };
            let err = FT_Get_SubGlyph_Info(self.raw(), sub_index, &index, &flags, &arg1, &arg2, &transfrom);
            if err == 0 {
                Ok((index, flags, arg1, arg2, transfrom))
            } else {
                Err(FromPrimitive::from_i32(err).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn raw<'a>(&'a self) -> &'a FT_GlyphSlotRec {
        unsafe {
            &*self.raw
        }
    }
}
