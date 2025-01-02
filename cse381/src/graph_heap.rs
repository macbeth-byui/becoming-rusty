use std::collections::HashMap;


/* Implement a binary heap to support a Graph.  Each node in the heap will contain the distance (f64) and the  
 * vertex (usize).  The heap is implemented as an array to provide faster access to the end of the heap.  A 
 * lookup table is implemented to provide quick access to a node in the heap to allow for changing the 
 * the distance.
*/
struct Node {
    distance : f64,
    vertex : usize
}

pub struct GraphHeap {
    heap : Vec<Node>,
    lookup : HashMap<usize, usize>
}

impl Default for GraphHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphHeap {
    /* Create an empty heap.
     */
    pub fn new() -> Self {
        let heap = Vec::<Node>::new();
        let lookup = HashMap::<usize, usize>::new();
        Self {heap, lookup}
    }

    /* Get the parent node based on index.  Returns None
     * if the index is not valid or there is no parent.
     */
    fn get_parent(&self, index : usize) -> Option<usize> {
        if index == 0 || index >= self.size() {
            return None;
        }
        Some((index - 1) / 2)
    }

    /* Get the left node based on index.  Returns None
     * if the index is invalid or there is no left child
     */
    fn get_left(&self, index : usize) -> Option<usize> {
        if index >= self.size() {
            return None;
        }        
        let left = (index * 2) + 1;
        if left >= self.size() { None } else { Some(left) }
    }

    /* Get the right node based on index.  Returns None
     * if the index is invalid or there is no right child.
     */
    fn get_right(&self, index : usize) -> Option<usize> {
        if index >= self.size() {
            return None;
        }
        let right = (index * 2) + 2;
        if right >= self.size() { None } else { Some(right) }
    }

    /* Bubble up a node so that the distance of a parent is always
     * less than or equal to the distance of the children.
     */
    fn bubble_up(&mut self, mut curr : usize) {
        loop {
            match self.get_parent(curr) {
                Some(parent) => {
                    // Properly placed already
                    if self.heap[parent].distance <= self.heap[curr].distance {
                        return;
                    }
                    // Move up
                    self.bubble_apply(&mut curr, parent);
                }
                None => return  // No Parent (must be the root or empty)
            }
        }
    }

    /* Bubble down a node so that the distance of a parent is always 
     * less than or equal to the distance of the children.
     */
    fn bubble_down(&mut self, mut curr : usize) {
        loop {
            match (self.get_left(curr), self.get_right(curr)) {
                (Some(left), Some(right)) => {
                    // Properly placed already
                    if self.heap[curr].distance <= self.heap[left].distance &&
                       self.heap[curr].distance <= self.heap[right].distance {
                        return;
                    }
                    // Move down to the left
                    if self.heap[left].distance <= self.heap[right].distance {
                        self.bubble_apply(&mut curr, left);
                    } 
                    // Move down to the right
                    else {
                        self.bubble_apply(&mut curr, right);
                    }
                }
                (Some(left), None) => {
                    // Properly placed already
                    if self.heap[curr].distance <= self.heap[left].distance {
                        return;
                    }
                    // Move down to the left
                    self.bubble_apply(&mut curr, left)
                }
                (None, Some(right)) => {
                    // Properly placed already
                    if self.heap[curr].distance <= self.heap[right].distance {
                        return;
                    }
                    // Move down to the right
                    self.bubble_apply(&mut curr, right)
                }
                (None, None) => return // No children (must be leaf)
            }
        }

    }

    /* Utility to swap, update lookup, and move current for the bubble functions
     */
    fn bubble_apply(&mut self, curr : &mut usize, target : usize) {
        // Swap, update lookup, and move 
        self.heap.swap(target, *curr);
        self.lookup.insert(self.heap[target].vertex, target);
        self.lookup.insert(self.heap[*curr].vertex, *curr);
        *curr = target;
    }

    /* Reduce the distance and move the node up.  Returns None if distance
     * did not go down or if the vertex is invalid.
     */
    pub fn decrease_distance(&mut self, vertex : usize, distance : f64) -> Option<()> {
        if let Some(curr) = self.lookup.get(&vertex) {
            if distance >= self.heap[*curr].distance {
                return None;
            }
            // Change the distance and move up
            self.heap[*curr].distance = distance;
            self.bubble_up(*curr);
            return Some(());
        }       
        None
    }

    /* Add a new node to the heap.
     */
    pub fn enqueue(&mut self, vertex : usize, distance : f64) {
        // Add to the end and move up as needed
        self.heap.push(Node {vertex, distance});
        self.lookup.insert(vertex, self.heap.len() -1);
        self.bubble_up(self.heap.len() - 1);
    }

    /* Remove the root node.  Return None if heap is already empty.
     */
    pub fn dequeue(&mut self) -> Option<usize> {
        if self.heap.is_empty() {
            return None;
        }
        // Swap last and first
        let vertex = self.heap[0].vertex;
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        self.lookup.insert(self.heap[0].vertex, 0);

        // Remove last
        self.heap.pop();
        self.lookup.remove(&vertex);

        // Move first down as needed
        self.bubble_down(0);
        Some(vertex)
    }

    /* Return size of the heap
     */
    pub fn size(&self) -> usize {
        self.heap.len()
    }


}