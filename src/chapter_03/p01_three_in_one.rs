//!
//! # Three in One:
//!
//! Describe how you could use a single array to implement three stacks.
//!
//! Hints: #2, #72, #38, #58
//!

/// Fixed-Size Multi-Stack
pub struct MultiStack {
    /// Number of stacks held
    num_stacks: usize,
    /// Size of each stack
    stack_size: usize,
    /// Items for all stacks
    items: Vec<isize>,
    /// Current sizes for each stack
    sizes: Vec<usize>,
}
impl MultiStack {
    /// Create a new [MultiStack] with `num_stacks` stacks,
    /// each of size `stack_size`.
    pub fn new(stack_size: usize, num_stacks: usize) -> Self {
        Self {
            items: vec![0; stack_size * num_stacks],
            sizes: vec![0; num_stacks],
            num_stacks,
            stack_size,
        }
    }
    /// Boolean indication of whether stack `stack_num` is empty.
    /// Returns a [StackError] if `stack_num` is invalid.
    pub fn is_empty(&self, stack_num: usize) -> StackResult<bool> {
        if stack_num >= self.num_stacks {
            return Err(StackError);
        }
        Ok(self.sizes[stack_num] == 0)
    }
    /// Boolean indication of whether stack `stack_num` is full.
    /// Returns a [StackError] if `stack_num` is invalid.
    pub fn is_full(&self, stack_num: usize) -> StackResult<bool> {
        if stack_num >= self.num_stacks {
            return Err(StackError);
        }
        Ok(self.sizes[stack_num] == self.stack_size)
    }
    /// Peek at the head of stack `stack_num`.
    /// Returns a [StackError] if empty, or if `stack_num` is invalid.
    pub fn peek(&self, stack_num: usize) -> StackResult<&isize> {
        if stack_num >= self.num_stacks || self.sizes[stack_num] == 0 {
            return Err(StackError);
        }
        Ok(&self.items[stack_num * self.stack_size])
    }
    /// Pop the head of stack `stack_num`.
    /// Returns a [StackError] if empty, or if `stack_num` is invalid.
    pub fn pop(&mut self, stack_num: usize) -> StackResult<isize> {
        if self.is_empty(stack_num)? {
            return Err(StackError);
        }
        let (start, end) = self.range(stack_num);

        // Grab our ultimate return value from the head
        let rv = self.items[start];

        // Rotate the remainder left by one
        let rot = &mut self.items[start..end];
        rot.rotate_left(1);
        // And decrement this stack's size
        self.sizes[stack_num] -= 1;
        Ok(rv)
    }
    /// Push value `val` to stack `stack_num`.
    /// Returns a [StackError] if full, or if `stack_num` is invalid.
    pub fn push(&mut self, stack_num: usize, val: isize) -> StackResult<()> {
        if stack_num >= self.num_stacks || self.sizes[stack_num] >= self.stack_size {
            return Err(StackError);
        }
        let (start, end) = self.range(stack_num);

        // Initially put the new value at the end of this stack
        self.items[end] = val;

        // Right-rotate it into the head-spot
        let rot = &mut self.items[start..end + 1];
        rot.rotate_right(1);

        self.sizes[stack_num] += 1;
        Ok(())
    }
    /// Get the range of indices occupied by stack `stack_num`
    /// Range is (inclusive, exclusive), per Rust norm.
    /// Panics if `stack_num` is out of bounds.
    fn range(&self, stack_num: usize) -> (usize, usize) {
        let start = stack_num * self.stack_size;
        let end = start + self.sizes[stack_num];
        (start, end)
    }
}

#[test]
fn test_three_in_one() {
    let num_stacks = 3;
    let stack_size = 6;
    let mut s = MultiStack::new(stack_size, num_stacks);

    for stack_num in 0..num_stacks {
        assert_eq!(s.is_empty(stack_num), Ok(true));
        assert_eq!(s.is_full(stack_num), Ok(false));
        assert_eq!(s.pop(stack_num), Err(StackError));

        for i in 0..(stack_size - 1) {
            s.push(stack_num, i as isize).unwrap();
            assert_eq!(s.peek(stack_num), Ok(&(i as isize)));
            assert_eq!(s.is_empty(stack_num), Ok(false));
            assert_eq!(s.is_full(stack_num), Ok(false));
        }

        s.push(stack_num, 999).unwrap();
        assert_eq!(s.push(stack_num, 777), Err(StackError));

        assert_eq!(s.is_empty(stack_num), Ok(false));
        assert_eq!(s.is_full(stack_num), Ok(true));
        assert_eq!(s.peek(stack_num), Ok(&999));
        assert_eq!(s.pop(stack_num), Ok(999));
        assert_eq!(s.is_empty(stack_num), Ok(false));
        assert_eq!(s.is_full(stack_num), Ok(false));

        for i in (1..(stack_size - 1)).rev() {
            assert_eq!(s.peek(stack_num), Ok(&(i as isize)));
            assert_eq!(s.pop(stack_num), Ok(i as isize));
            assert_eq!(s.is_empty(stack_num), Ok(false));
            assert_eq!(s.is_full(stack_num), Ok(false));
        }

        assert_eq!(s.peek(stack_num), Ok(&0_isize));
        assert_eq!(s.pop(stack_num), Ok(0));
        assert_eq!(s.is_empty(stack_num), Ok(true));
        assert_eq!(s.is_full(stack_num), Ok(false));

        assert_eq!(s.peek(stack_num), Err(StackError));
        assert_eq!(s.pop(stack_num), Err(StackError));
    }

    let mut s = MultiStack::new(3, 1);
    assert_eq!(s.push(1, 1), Err(StackError));
}

/// Local Error Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackError;

/// Corresponding Result type-alias
pub type StackResult<T> = Result<T, StackError>;
