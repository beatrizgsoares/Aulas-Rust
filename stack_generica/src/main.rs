#[derive(Debug)]
struct Stack<T>{
    stack: Vec<T>,
}
fn main() {
    let mut si32  = Stack::new();
    let mut su32  = Stack::new();
    let mut schar  = Stack::new();
    let mut sstr  = Stack::new();

    si32.push(1i32);
    su32.push(2u32);
    schar.push('a');
    sstr.push("abc");

    println!("{:?}", si32);
    println!("{:?}", su32);
    println!("{:?}", schar);
    println!("{:?}", sstr);

    si32.pop();
    su32.pop();
    schar.pop();
    sstr.pop();

    println!("{:?}", si32);
    println!("{:?}", su32);
    println!("{:?}", schar);
    println!("{:?}", sstr);

    si32.append(vec![1,2,3,4,5]);
    su32.append(vec![6,7,8]);
    schar.append(vec!['a','b','c','d','e','f']);
    sstr.append(vec!["abc", "def", "ghi"]);

    println!("{:?}, len: {}", si32, si32.len());
    println!("{:?}, len: {}", su32, su32.len());
    println!("{:?}, len: {}", schar, schar.len());
    println!("{:?}, len: {}", sstr, sstr.len());

    println!("{:?}", si32.peek());
    println!("{:?}", su32.peek());
    println!("{:?}", schar.peek());
    println!("{:?}", sstr.peek());
}
impl <T>Stack<T>{
    fn new() -> Stack<T>{
        Stack{stack: vec![]}
    }
    fn push(&mut self, val: T){
        self.stack.push(val);
    }
    fn pop(&mut self) -> Option<T>{
        self.stack.pop()
    }
    fn len(&self) -> usize{
        self.stack.len()
    }
    fn append(&mut self, mut vetor: Vec<T>){
        self.stack.append(&mut vetor);
    }
    fn peek(&self) -> Option<&T>{
        self.stack.last()
    }
}