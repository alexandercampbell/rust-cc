
/**
 * StepbackIterator is an Iterator that provides a `step_back()` in addition to the standard
 * `next()` method, for any vector of items.
 *
 * This is very useful in lexing and parsing.
 */
pub struct StepbackIterator<T: Clone> {
    items:  Vec<T>,
    pos:    usize,
}

impl<T: Clone> Iterator for StepbackIterator<T> {
    type Item = T;

    /**
     * StepbackIterator will continue to advance its index every time you call this function, even
     * beyond the end of the array. This means that `step_back()` will always behave like you
     * expect.
     *
     * ```
     * let s = StepbackIterator::new(vec![1, 2]);
     * s.next(); // returns 1
     * s.next(); // returns 2
     * s.next(); // returns None
     * s.next(); // returns None
     * s.step_back();
     *
     * // Returns None, because we have not stepped back to the end of the list yet; our internal
     * // index is still pointing to `len() + 1`.
     * s.next();
     * ```
     *
     */
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.items.len() {
            self.pos += 1; // so step_back does exactly what we expect every time
            None
        } else {
            let next_item = self.items[self.pos].clone();
            self.pos += 1;
            Some(next_item)
        }
    }
}

impl<T: Clone> StepbackIterator<T> {
    /**
     * Create a new StepbackIterator from the items you provide. If you have an existing Iterator,
     * I would recommend calling `Iterator#exhaust()` first. StepbackIterator depends on having a
     * complete vector of items.
     */
    pub fn new(items: Vec<T>) -> Self {
        StepbackIterator{
            items:  items,
            pos:    0,
        }
    }

    /**
     * Move one item backward in the Iterator. This has an effect opposite that of `next()`. This
     * is the important part of StepbackIterator.
     */
    pub fn step_back(&mut self) {
        if self.pos > 0 { self.pos -= 1; }
    }

    /**
     * Technically, since StepbackIterator implements Iterator, you could do `.peekable().peek()`.
     * However, to prevent the overhead associated with creating a new struct, I've provided peek
     * here directly.
     */
    pub fn peek(&self) -> Option<T> {
        if self.pos >= self.items.len() {
            None
        } else {
            Some(self.items[self.pos].clone())
        }
    }

    /**
     * Returns true if the end of the item stream has been reached.
     */
    pub fn is_exhausted(&self) -> bool {
        self.pos >= self.items.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_and_step_back() {
        let mut i = StepbackIterator::new(vec![1, 2, 3]);
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert!(!i.is_exhausted());
        assert_eq!(i.next(), Some(3));
        assert!(i.is_exhausted());
        assert_eq!(i.next(), None);
        assert!(i.is_exhausted());
        i.step_back();
        i.step_back();
        assert!(!i.is_exhausted());
        assert_eq!(i.next(), Some(3));
    }
}

