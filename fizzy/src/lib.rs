use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    predicate: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T> where F: Fn(T) -> bool + 'static, S: ToString {
        Self { predicate: Box::new(matcher), sub: subs.to_string() }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matches: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self { matches: Vec::new() }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matches.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item=String>
    where
        I: Iterator<Item=T>,
    {
        iter.map(move |item| {
            let matches = self.matches.iter()
                .filter(|x| (x.predicate)(item))
                .map(|x| x.sub.clone())
                .collect::<String>();
            if matches.is_empty() {
                item.to_string()
            } else { matches }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where T: Rem<Output = T> + PartialEq + From<u8> + Copy {
    let matchers = vec![
        Matcher::new(|n| n % T::from(3u8) == T::from(0u8), "fizz"),
        Matcher::new(|n| n % T::from(5u8) == T::from(0u8), "buzz"),
    ];

    Fizzy { matches: matchers }
}
