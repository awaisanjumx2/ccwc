use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Count bytes in the file
    #[arg(short = 'c')]
    pub bytes: bool,

    /// Count lines in the file
    #[arg(short = 'l')]
    pub lines: bool,

    /// Count words in the file
    #[arg(short = 'w')]
    pub words: bool,
    
    /// Count characters in the file
    #[arg(short = 'm')]
    pub characters: bool,

    /// Input file
    pub file: String,
}

impl Args {
    pub fn no_flags_passed(&self) -> bool {
        self.bytes == false && self.lines == false && self.words == false && self.characters == false
    }
}