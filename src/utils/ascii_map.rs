use glam::{UVec2, uvec2};

#[inline]
pub fn parse_ascii_map(input: &str) -> impl Iterator<Item = (UVec2, u8)> + Clone + '_ {
    debug_assert!(input.is_ascii());
    input.lines().enumerate().flat_map(|(y, line)| {
        let y: u32 = y.try_into().unwrap();
        line.bytes().enumerate().map(move |(x, b)| {
            let x: u32 = x.try_into().unwrap();
            (uvec2(x, y), b)
        })
    })
}

#[inline]
pub fn ascii_map_size(input: &str) -> UVec2 {
    debug_assert!(input.is_ascii());

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    debug_assert!(
        input.lines().all(|line| line.len() == width),
        "some lines of `input` haven't the same length"
    );

    uvec2(width.try_into().unwrap(), height.try_into().unwrap())
}
