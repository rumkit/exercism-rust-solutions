use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type NodePointerOption<T> = Option<NodePointer<T>>;
type NodePointer<T> = NonNull<Node<T>>;

pub struct LinkedList<T> {
    head: NodePointerOption<T>,
    tail: NodePointerOption<T>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: NodePointerOption<T>,
    prev: NodePointerOption<T>,
}

pub struct Cursor<'a, T> {
    current: NodePointerOption<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    current: NodePointerOption<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor { current: self.head, list: self }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor { current: self.tail, list: self }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { current: self.head, _marker: Default::default() }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        if self.is_empty() {
            return;
        }
        let mut next = self.head;
        unsafe {
            while let Some(to_drop) = next {
                next = to_drop.as_ref().next;
                drop(Box::from_raw(to_drop.as_ptr()));
            }
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}

unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element,
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current.map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        if let Some(node) = self.current {
            unsafe {
                self.current = node.as_ref().next;
                return self.current.map(|node | &mut (*node.as_ptr()).element)
            }
        }
        None
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if let Some(node) = self.current {
            unsafe {
                self.current = node.as_ref().prev;
                return self.current.map(|node | &mut (*node.as_ptr()).element)
            }
        }
        None
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let node = self.current?;

        unsafe {
            let next_ptr = node.as_ref().next;
            let prev_ptr = node.as_ref().prev;

            //  If adjacent nodes exist, relocate pointers
            if let Some(mut prev) = prev_ptr {
                prev.as_mut().next = next_ptr;
            } else {
                // Current node was the list's head
                self.list.head = next_ptr;
            }
            if let Some(mut next) = next_ptr {
                next.as_mut().prev = prev_ptr;
            } else {
                // Current node was the list's tail
                self.list.tail = prev_ptr;
            }

            // Move cursor
            if next_ptr.is_some() { self.current = next_ptr; } else { self.current = prev_ptr; }

            self.list.len -= 1;
            let boxed_node = Box::from_raw(node.as_ptr());
            Some(boxed_node.element)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if let Some((mut new_node, mut current)) = self.prepare_insert(element) {
            let node_ptr = Some(new_node);
            unsafe {
                new_node.as_mut().prev = Some(current);
                if let Some(mut next) = current.as_ref().next {
                    next.as_mut().prev = node_ptr;
                    new_node.as_mut().next = Some(next);
                } else {
                    self.list.tail = node_ptr;
                }
                current.as_mut().next = node_ptr;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if let Some((mut new_node, mut current)) = self.prepare_insert(element) {
            let node_ptr = Some(new_node);
            unsafe {
                new_node.as_mut().next = Some(current);
                if let Some(mut prev) = current.as_ref().prev {
                    prev.as_mut().next = node_ptr;
                    new_node.as_mut().prev = Some(prev);
                } else {
                    self.list.head = node_ptr;
                }
                current.as_mut().prev = node_ptr;
            }
        }
    }

    /// Handles node allocation, empty-list insertion, and validation.
    /// Returns `Some((new_node, current_node))` if further unsafe pointer stitching is required.
    fn prepare_insert(&mut self, element: T) -> Option<(NodePointer<T>, NodePointer<T>)> {
        if self.list.is_empty() {
            let new_node = NonNull::from(Box::leak(Box::new(Node {
                element,
                next: None,
                prev: None
            })));
            let node_ptr = Some(new_node);

            self.list.head = node_ptr;
            self.list.tail = node_ptr;
            self.current = node_ptr;
            self.list.len += 1;
            // Insert handled without needing of unsafe part
            None
        } else {
            // List is not empty and, if Cursor is out bounds, the code panics
            let current_node = self.current.expect("Cursor is not valid");
            // Allocation is done (again, but for this if-else branch for the first time)
            // after the point where code might panic, so no leaked memory is left behind
            let new_node = NonNull::from(Box::leak(Box::new(Node {
                element,
                next: None,
                prev: None
            })));

            self.list.len += 1;
            // Ready for unsafe pointer stitching 😷🪡
            Some((new_node, current_node))
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.current.map(|node| unsafe {
            let item = &node.as_ref().element;
            self.current = node.as_ref().next;
            item
        })
    }
}
