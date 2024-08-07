struct StackElem {
    val: i32,
    min: i32,
}
struct MinStack {
    stack: Vec<StackElem>,
    min: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }
    
    fn push(&mut self, val: i32) {
        let elem = StackElem {
            val,
            min: self.min,
        };
        self.stack.push(elem);
        self.min = val.min(self.min);
    }
    
    fn pop(&mut self) {
        let elem = self.stack.pop().unwrap();
        self.min = elem.min;
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

// /**
//  * Your MinStack object will be instantiated and called as such:
//  * let obj = MinStack::new();
//  * obj.push(val);
//  * obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: i32 = obj.get_min();
//  */