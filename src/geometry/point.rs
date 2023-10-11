#[derive(Clone, Copy, Debug, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "layout")]
mod with_layout {
    use super::Point;
    use taffy::style::Dimension;

    impl<T> Point<T> {
        pub(crate) fn from_taffy(size: taffy::geometry::Point<T>) -> Self {
            Self {
                x: size.x,
                y: size.y,
            }
        }
    }

    impl Point<Dimension> {
        pub fn from_points(x: f32, y: f32) -> Self {
            Self {
                x: Dimension::Points(x),
                y: Dimension::Points(y),
            }
        }
    }
}
