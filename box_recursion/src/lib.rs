#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Role {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            role: Role::from(role),
            name: name.to_owned(),
            next: self.grade.take(),
        };

        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(first_worker) => {
                self.grade = first_worker.next;

                Some(first_worker.name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        match &self.grade {
            Some(worker) => Some((worker.name.clone(), worker.role)),
            None => None,
        }
    }
}
