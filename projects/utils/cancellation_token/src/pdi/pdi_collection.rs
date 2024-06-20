pub trait PDIItemIndicator<'a> {
    type CollectionType;

    fn remove(self);
}

pub trait PDICollection : IntoIterator<Item=<Self as PDICollection>::Item> + Default
{
    type Item;
    type Indicator<'a>: PDIItemIndicator<'a, CollectionType = Self> where Self: 'a;

    fn push(&mut self, item: <Self as PDICollection>::Item) -> Self::Indicator<'_>;

    fn swap(&mut self, other: &mut Self);
}
