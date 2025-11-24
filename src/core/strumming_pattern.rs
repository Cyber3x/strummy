use std::{
    fs::File,
    io::{self, Read, Write},
};

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StrummingPattern {
    pub strokes: Vec<Stroke>,
}

impl StrummingPattern {
    pub fn new_random(len: usize) -> Self {
        let mut rng = rand::rng();

        let choices = [Stroke::Down, Stroke::Up, Stroke::Mute, Stroke::Miss];

        let strokes = (0..len)
            .map(|_| *choices.choose(&mut rng).unwrap())
            .collect();

        Self { strokes }
    }

    pub fn len(&self) -> usize {
        self.strokes.len()
    }

    pub fn set_stroke(&mut self, index: usize, stroke: Stroke) {
        self.strokes[index] = stroke;
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let pattern: StrummingPattern = serde_json::from_str(&contents)?;
        Ok(pattern)
    }
}

impl std::fmt::Display for StrummingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<_> = self.strokes.iter().map(|s| s.shorthand()).collect();

        write!(f, "{}", parts.join(" "))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Stroke {
    Up,
    Down,
    Mute,
    Miss,
}

impl Stroke {
    pub fn shorthand(&self) -> &'static str {
        match self {
            Stroke::Down => "D",
            Stroke::Up => "U",
            Stroke::Mute => "x",
            Stroke::Miss => "-",
        }
    }
}
