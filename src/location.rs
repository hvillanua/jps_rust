use std::ops;

use overload::overload;

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn direction(&self) -> Location {
        let x = if self.x > 0 {
            1
        } else if self.x < 0 {
            -1
        } else {
            0
        };
        let y = if self.y > 0 {
            1
        } else if self.y < 0 {
            -1
        } else {
            0
        };
        Location { x, y }
    }
}

overload!((a: ?Location) + (b: ?Location) -> Location { Location{ x: a.x + b.x, y: a.y + b.y } });
overload!((a: &mut Location) += (b: ?Location) { *a = Location{ x: a.x + b.x, y: a.y + b.y } });
overload!((a: ?Location) - (b: ?Location) -> Location { Location{ x: a.x - b.x, y: a.y - b.y } });
overload!((a: &mut Location) -= (b: ?Location) { *a = Location{ x: a.x - b.x, y: a.y - b.y } });
overload!((a: ?Location) * (b: ?i32) -> Location { Location{ x: a.x * b, y: a.y * b } });
overload!((a: ?i32) * (b: ?Location) -> Location { Location{ x: b.x * a, y: b.y * a } });
overload!((a: &mut Location) *= (b: ?i32) { *a = Location{ x: a.x * b, y: a.y * b } });
overload!(- (a: ?Location) -> Location { Location{ x: -a.x, y: -a.y } });
