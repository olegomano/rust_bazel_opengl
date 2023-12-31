use std::fmt::Write;

struct Builder(String);

impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(&mut self.0, "M {} {} ", x, y).unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(&mut self.0, "L {} {} ", x, y).unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(&mut self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    }

    fn close(&mut self) {
        write!(&mut self.0, "Z ").unwrap();
    }
}

#[test]
fn endless_loop() {
    let data = b"\x00\x01\x00\x00\x00\x0f\x00\x10\x00PTT-W\x002h\xd7\x81x\x00\
    \x00\x00?L\xbaN\x00c\x9a\x9e\x8f\x96\xe3\xfeu\xff\x00\xb2\x00@\x03\x00\xb8\
    cvt 5:\x00\x00\x00\xb5\xf8\x01\x00\x03\x9ckEr\x92\xd7\xe6\x98M\xdc\x00\x00\
    \x03\xe0\x00\x00\x00dglyf\"\t\x15`\x00\x00\x03\xe0\x00\x00\x00dglyf\"\t\x15\
    `\x00\x00\x00 \x00\x00\x00\xfc\x97\x9fmx\x87\xc9\xc8\xfe\x00\x00\xbad\xff\
    \xff\xf1\xc8head\xc7\x17\xce[\x00\x00\x00\xfc\x00\x00\x006hhea\x03\xc6\x05\
    \xe4\x00\x00\x014\x00\x00\x00$hmtx\xc9\xfdq\xed\x00\x00\xb5\xf8\x01\x00\x03\
    \x9ckEr\x92\xd7\xe6\xdch\x00\x00\xc9d\x00\x00\x04 loca\x00M\x82\x11\x00\x00\
    \x00\x06\x00\x00\x00\xa0maxp\x17\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00 name\
    \xf4\xd6\xfe\xad\x00OTTO\x00\x02gpost5;5\xe1\x00\x00\xb0P\x00\x00\x01\xf0perp%\
    \xb0{\x04\x93D\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x01\x00\x00\xe1!yf%1\
    \x08\x95\x00\x00\x00\x00\x00\xaa\x06\x80fmtx\x02\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00a\xcc\xff\
    \xce\x03CCCCCCCCC\x00\x00\x00\x00\x00C\x00\x00\x00\x00\xb5\xf8\x01\x00\x00\x9c";

    let face = ttf_parser::Face::parse(data, 0).unwrap();
    let _ = face.outline_glyph(ttf_parser::GlyphId(0), &mut Builder(String::new()));
}
