use rowan::TextSize;

pub trait Locate {
    type Item;

    fn locate(&self, offset: TextSize) -> Option<Self::Item>;
}
