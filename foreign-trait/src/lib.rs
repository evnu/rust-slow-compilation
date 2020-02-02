#[derive(Copy, Clone)]
pub struct Term<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

pub fn get_tuple<'a>(_term: Term<'a>) -> Result<Vec<Term<'a>>, ()> {
    Ok(vec![])
}

pub trait Decoder<'a>: Sized + 'a {
    fn decode(t: Term<'a>) -> Result<Self, ()>;
}

impl<'a> Decoder<'a> for i64 {
    fn decode(_: Term<'a>) -> Result<Self, ()> {
        Ok(0)
    }
}
