use bumpalo::Bump;
use std::mem;

pub trait View<'a> {
    type Element;

    fn build(&'a self) -> Self::Element;

    fn rebuild(&'a self, element: &mut Self::Element);
}

pub struct Tree<V> {
    component: fn(&Bump) -> V,
    frame_a: Bump,
    frame_b: Bump,
    is_frame_a: bool,
}

impl<V> Tree<V> {
    pub fn new<'a>(component: fn(&'a Bump) -> V) -> Self
    where
        V: View<'a> + 'a,
    {
        let component = unsafe {
             mem::transmute(component)
        };
        Self {
            component,
            frame_a: Bump::new(),
            frame_b: Bump::new(),
            is_frame_a: true,
        }
    }

    pub fn view<'a>(&mut self)
    where
        V: View<'a> + 'a,
    {
        let bump = if self.is_frame_a {
            self.is_frame_a = false;
            &self.frame_a
        } else {
            self.is_frame_a = true;
            &self.frame_b
        };
        let bump: &'a Bump = unsafe { mem::transmute(bump) };
        
        let view = (self.component)(bump);       
        bump.alloc(view).build();
    }
    
}

impl<'a> View<'a> for &'a str {
    type Element = ();

    fn build(&'a self) -> Self::Element {
        dbg!(self);
    }

    fn rebuild(&'a self, element: &mut Self::Element) {
        dbg!(self);
    }
}
