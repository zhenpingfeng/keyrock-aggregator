use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub fn deserialize_first_n<'de, T, D, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct TakeVisitor<T, const N: usize>(PhantomData<fn() -> T>);

    impl<'de, T, const N: usize> de::Visitor<'de> for TakeVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "array of at least {} elements", N)
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<[T; N], S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            // SAFETY: Assuming array of MaybeUninit instances is initialized is safe because
            // MaybeUninit does not require initialization.
            let mut items: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

            for i in 0..N {
                let value = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("not enough items in array"))?;
                items[i].write(value);
            }

            // Skip any remaining elements
            while let Some(de::IgnoredAny) = seq.next_element()? {
                // ignore
            }

            Ok(items.map(|item| unsafe { item.assume_init() }))
        }
    }

    let visitor = TakeVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

pub fn deserialize_first_10<'de, T, D>(deserializer: D) -> Result<[T; 10], D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    deserialize_first_n(deserializer)
}
