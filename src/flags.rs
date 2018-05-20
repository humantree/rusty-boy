#[derive(Debug)]
pub struct Flags {
    pub z:  bool,
    pub n:  bool,
    pub h:  bool,
    pub cy: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z:  false,
            n:  false,
            h:  false,
            cy: false,
        }
    }
}
