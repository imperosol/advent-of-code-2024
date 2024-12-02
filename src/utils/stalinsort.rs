use std::iter::Peekable;

pub struct StalinSortIterBy<I: Iterator, F: Fn(&I::Item, &I::Item) -> bool> {
    cmp: F,
    inner: Peekable<I>,
}

impl<I, F> Iterator for StalinSortIterBy<I, F>
where
    I: Iterator,
    I::Item: Ord,
    F: Fn(&I::Item, &I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        next_adapter(&mut self.inner, &self.cmp)
    }
}
pub struct StalinSortIter<I: Iterator> {
    inner: Peekable<I>,
}

impl<I> Iterator for StalinSortIter<I>
where
    I: Iterator,
    I::Item: Ord,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        next_adapter(&mut self.inner, |a, b| a.le(b))
    }
}

pub trait StalinSort: Iterator {
    /// Purge all non-sorted elements.
    ///
    /// ```
    /// use adventofcode::utils::StalinSort;
    ///
    /// // sort people in descending order by age
    /// let numbers = vec![1, 3, 4, 3, 5, 1, 2, 3];
    ///
    /// let ordered = numbers.iter().stalin_sorted().collect::<Vec<_>>();
    ///
    /// assert_eq!(ordered, vec![1, 3, 4, 5]);
    /// ```
    fn stalin_sorted(self) -> StalinSortIter<Self>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        StalinSortIter {
            inner: self.peekable(),
        }
    }

    /// Purge all non-sorted elements, using the given comparison function
    ///
    /// ```
    /// use adventofcode::utils::StalinSort;
    ///
    /// // sort people by age
    /// let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 30)];
    ///
    /// let ordered_by_age = people
    ///     .into_iter()
    ///     .stalin_sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
    ///     .map(|(person, _age)| person)
    ///     .collect::<Vec<_>>();
    ///
    /// // John is in gulag now :)
    /// assert_eq!(ordered_by_age, vec!["Jane", "Jill", "Jack"]);
    /// ```
    fn stalin_sorted_by<F>(self, cmp: F) -> StalinSortIterBy<Self, F>
    where
        Self: Sized,
        Self::Item: Ord,
        F: Fn(&Self::Item, &Self::Item) -> bool,
    {
        StalinSortIterBy {
            inner: self.peekable(),
            cmp,
        }
    }
}

impl<T> StalinSort for T where T: Iterator + ?Sized {}

fn next_adapter<I, F>(iter: &mut Peekable<I>, cmp: F) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Ord,
    F: Fn(&I::Item, &I::Item) -> bool,
{
    let curr = iter.next();
    if let Some(curr) = &curr {
        while let Some(next) = iter.peek() {
            if !cmp(curr, next) {
                iter.next().unwrap();
            } else {
                break;
            }
        }
    }
    curr
}
