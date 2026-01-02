// ScaleFactor /////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct ScaleFactor{
    pub scale  : f64,
    pub dst_w  : i32, pub dst_h  : i32,
    pub ofst_x : i32, pub ofst_y : i32,
}
impl ScaleFactor{
    pub fn get_scale_offset(src_w: i32, src_h: i32, dst_w: i32, dst_h: i32 ) -> Self{
        let scale = (dst_w as f64) / (src_w as f64);
        if ((src_h as f64) * scale) <= (dst_h as f64) {
            let ofst_y = (((dst_h as f64) - ((src_h as f64) * scale)) / 2.0) as i32;
            return Self{
                scale: scale,
                dst_w: dst_w,
                dst_h: ((src_h as f64) * scale) as i32,
                ofst_x: 0, ofst_y: ofst_y};
        }
        let scale = (dst_h as f64) / (src_h as f64);
        let ofst_x = (((dst_w as f64) - ((src_w as f64) * scale)) / 2.0) as i32;
        return Self{
            scale: scale,
            dst_w: ((src_w as f64) * scale) as i32,
            dst_h: dst_h,
            ofst_x: ofst_x, ofst_y: 0
        };
    }
}
