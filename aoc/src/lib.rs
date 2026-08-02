use std::path::Path;

/// Read `<crate>/inputs/dayNN.txt`. Use via the [`input!`] macro.
pub fn read_input(manifest_dir: &str, day: u32) -> String {
    let path = Path::new(manifest_dir).join(format!("inputs/day{day:02}.txt"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// `let input = aoc::input!(1);` -> contents of `inputs/day01.txt` in the calling crate.
#[macro_export]
macro_rules! input {
    ($day:expr) => {
        $crate::read_input(env!("CARGO_MANIFEST_DIR"), $day)
    };
}
