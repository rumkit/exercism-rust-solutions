use std::iter::from_fn;

/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    from_fn(move || a.next().or_else(|| b.next()))
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut internal_iter = None;

    from_fn(move || {
        loop {
            if internal_iter.is_none() {
                internal_iter = nested_iter.next();
            }

            // Borrow the inner iterator mutably
            if let Some(ref mut inner) = internal_iter {
                if let Some(item) = inner.next() {
                    break Some(item);
                }
                // inner iterator is exhausted → drop it and continue loop
                internal_iter = None;
                continue;
            } else {
                break None;
            }
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    from_fn(
        move || {
            loop {
                match iter.next() {
                    Some(item) if predicate(&item) => break Some(item),
                    Some(_) => continue,
                    None => break None,
                }
            }
        }
    )
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    from_fn(
        move || {
            iter.next().map(&function)
        }
    )
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for item in iter {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    foldl(reverse(iter), initial, function)
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    from_fn(move || iter.next_back())
}
