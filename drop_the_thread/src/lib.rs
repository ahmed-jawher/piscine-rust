use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

pub type Workers = ThreadPool;

impl ThreadPool {
    pub fn new() -> ThreadPool {
        ThreadPool {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread<'_>) {
        let pid = self.thread_len();
        self.states.borrow_mut().push(false);
        (pid, Thread::new(pid, c, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        self.drops.set(self.drops.get() + 1);
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread<'_>) {
        self.new_thread(c)
    }

    pub fn track_worker(&self) -> usize {
        self.thread_len()
    }

    pub fn add_drop(&self, id: usize) {
        self.drop_thread(id)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(pid: usize, cmd: String, parent: &'a ThreadPool) -> Thread<'a> {
        Thread { pid, cmd, parent }
    }

    pub fn new_thread(p: usize, c: String, t: &'a ThreadPool) -> Thread<'a> {
        Thread::new(p, c, t)
    }

    pub fn skill(self) {}
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}
