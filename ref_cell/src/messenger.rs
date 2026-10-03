use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        *self.value.borrow_mut() = count;
        let percentage = count * 100 / self.max;

        if percentage > 100 {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_owned());
        } else if percentage >= 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let percentage = Rc::strong_count(value) * 100 / self.max;
        self.messages
            .borrow_mut()
            .push(format!("Info: This value would use {}% of your quota", percentage));
    }
}
