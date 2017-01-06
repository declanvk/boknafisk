bitflags! {
    pub flags CastlingFlags: u8 {
        const WHITE_QUEENSIDE = 0b0000001,
        const WHITE_KINGSIDE  = 0b0000010,
        const BLACK_QUEENSIDE = 0b0000100,
        const BLACK_KINGSIDE  = 0b0001000
    }
}

pub enum CastleType {
    Kingside,
    Queenside,
}
