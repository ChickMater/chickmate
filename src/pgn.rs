use pgn_reader::{RawHeader, SanPlus, Skip, Visitor};
use shakmaty::{fen::Fen, uci::Uci, CastlingMode, Chess, Position};
use std::str;
#[derive(Default)]
pub struct Game {
    pub white: String,
    pub black: String,
    pub url: String,
    pub moves: Vec<Uci>,
}

#[derive(Default)]
pub struct PgnParser {
    games: Vec<Game>,
    pos: Chess,
}

impl PgnParser {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn games(self) -> Vec<Game> {
        self.games
    }
}

impl Visitor for PgnParser {
    type Result = ();
    fn begin_game(&mut self) {
        self.games.push(Default::default());
        self.pos = Default::default();
    }
    fn end_game(&mut self) -> Self::Result {
        ()
    }
    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        if let (Ok(k), Ok(v)) = (str::from_utf8(key), value.decode_utf8()) {
            match k {
                "White" => self.games.last_mut().unwrap().white = v.into_owned(),
                "Black" => self.games.last_mut().unwrap().black = v.into_owned(),
                "Site" => self.games.last_mut().unwrap().url = v.into_owned(),
                "Fen" => {
                    self.pos = Fen::from_ascii(value.as_bytes())
                        .ok()
                        .and_then(|f| f.position(CastlingMode::Standard).ok())
                        .unwrap()
                }
                _ => (),
            }
        }
    }
    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }
    fn san(&mut self, san_plus: SanPlus) {
        let m = san_plus.san.to_move(&self.pos).unwrap();
        self.pos.play_unchecked(&m);
        self.games
            .last_mut()
            .unwrap()
            .moves
            .push(Uci::from_standard(&m));
    }
}
