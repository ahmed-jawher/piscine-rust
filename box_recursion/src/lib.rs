#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };

        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(first_worker) => {
                let removed_name = first_worker.name.clone();

                self.grade = first_worker.next;

                Some(removed_name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Some(worker) => Some((worker.name.clone(), worker.role.clone())),
            None => None,
        }
    }
}
