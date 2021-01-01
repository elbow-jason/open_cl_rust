use super::{Number, NumberType, NumberTypedT};

impl<T> NumberTypedT for T
where
    T: Number,
{
    fn number_type() -> NumberType {
        NumberType::new::<T>()
    }
}
