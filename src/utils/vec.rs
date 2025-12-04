use glam::{UVec2, uvec2};

/* -------------------------------------------------------------------------- */

#[inline]
pub fn four_directions_bounded(
    pos: glam::UVec2,
    bounds: glam::UVec2,
) -> impl Iterator<Item = glam::UVec2> {
    let mut directions = [None; 4];

    // left
    if let Some(x) = pos.x.checked_sub(1) {
        directions[0] = Some(uvec2(x, pos.y));
    }

    // top
    if let Some(y) = pos.y.checked_sub(1) {
        directions[1] = Some(uvec2(pos.x, y));
    }

    // right
    {
        let x = pos.x + 1;
        if x < bounds.x {
            directions[2] = Some(uvec2(x, pos.y));
        }
    }

    // down
    {
        let y = pos.y + 1;
        if y < bounds.y {
            directions[3] = Some(uvec2(pos.x, y));
        }
    }

    directions.into_iter().flatten()
}

#[inline]
pub fn four_directions(pos: glam::UVec2) -> impl Iterator<Item = glam::UVec2> + Clone {
    [pos.left(), pos.up(), Some(pos.right()), Some(pos.down())]
        .into_iter()
        .flatten()
}

#[inline]
pub fn height_directions_bounded(
    pos: glam::UVec2,
    bounds: glam::UVec2,
) -> impl Iterator<Item = glam::UVec2> {
    let mut directions = [None; 8];

    // left
    if let Some(x) = pos.x.checked_sub(1) {
        directions[0] = Some(uvec2(x, pos.y));
    }

    // top-left
    if let Some(x) = pos.x.checked_sub(1)
        && let Some(y) = pos.y.checked_sub(1)
    {
        directions[1] = Some(uvec2(x, y));
    }

    // top
    if let Some(y) = pos.y.checked_sub(1) {
        directions[2] = Some(uvec2(pos.x, y));
    }

    // top-right
    {
        let x = pos.x + 1;
        if x < bounds.x
            && let Some(y) = pos.y.checked_sub(1)
        {
            directions[3] = Some(uvec2(x, y));
        }
    }

    // right
    {
        let x = pos.x + 1;
        if x < bounds.x {
            directions[4] = Some(uvec2(x, pos.y));
        }
    }

    // down-right
    {
        let y = pos.y + 1;
        let x = pos.x + 1;
        if x < bounds.x && y < bounds.y {
            directions[5] = Some(uvec2(x, y));
        }
    }

    // down
    {
        let y = pos.y + 1;
        if y < bounds.y {
            directions[6] = Some(uvec2(pos.x, y));
        }
    }

    // down-left
    {
        let y = pos.y + 1;
        if let Some(x) = pos.x.checked_sub(1)
            && y < bounds.y
        {
            directions[7] = Some(uvec2(x, y));
        }
    }

    directions.into_iter().flatten()
}

/* -------------------------------------------------------------------------- */

pub trait VecExt {
    fn up(&self) -> Option<UVec2>;
    fn down(&self) -> UVec2;
    fn left(&self) -> Option<UVec2>;
    fn right(&self) -> UVec2;
}

impl VecExt for UVec2 {
    #[inline]
    fn up(&self) -> Option<UVec2> {
        self.y.checked_sub(1).map(|y| uvec2(self.x, y))
    }

    #[inline]
    fn down(&self) -> UVec2 {
        uvec2(self.x, self.y + 1)
    }

    #[inline]
    fn left(&self) -> Option<UVec2> {
        self.x.checked_sub(1).map(|x| uvec2(x, self.y))
    }

    #[inline]
    fn right(&self) -> UVec2 {
        uvec2(self.x + 1, self.y)
    }
}

/* -------------------------------------------------------------------------- */
