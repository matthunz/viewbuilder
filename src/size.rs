
#[derive(Clone, Copy, Debug, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

#[cfg(feature = "layout")]
mod with_layout {
    use taffy::style::Dimension;

    use crate::Size;

    impl Size<Dimension> {
        pub fn from_points(width: f32, height: f32) -> Self {
            Self {
                width: Dimension::Points(width),
                height: Dimension::Points(height),
            }
        }

        pub(crate) fn into_taffy(self) -> taffy::prelude::Size<Dimension> {
            taffy::prelude::Size {
                width: self.width,
                height: self.height,
            }
        }
    }
}
