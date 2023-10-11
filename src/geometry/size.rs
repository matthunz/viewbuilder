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
    use super::Size;
    use taffy::style::Dimension;

    impl<T> Size<T> {
        pub(crate) fn from_taffy(size: taffy::prelude::Size<T>) -> Self {
            Self {
                width: size.width,
                height: size.height,
            }
        }
    }

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
