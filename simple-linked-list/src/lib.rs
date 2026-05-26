pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut head = &self.head;

        while let Some(node) = head {
            count += 1;
            head = &node.next;
        }

        count
    }

    pub fn append(&mut self, element: T) {
        let mut tail = &mut self.head;

        while let Some(node) = tail {
            tail = &mut node.next;
        }

        *tail = Some(Box::new(Node {
            value: element,
            next: None,
        }));
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            value: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut head_node| {
            self.head = head_node.next.take();
            head_node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut head = self.head;

        let mut previous_node: Option<Box<Node<T>>> = None;
        while let Some(node) = head {
            previous_node = Some(Box::new(Node {
                value: node.value,
                next: previous_node,
            }));
            head = node.next;
        }
        SimpleLinkedList {
            head: previous_node,
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter.into_iter() {
            list.push(item);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        let mut rev_list = linked_list.rev();
        while let Some(node) = rev_list.pop() {
            vec.push(node)
        }
        vec
    }
}
