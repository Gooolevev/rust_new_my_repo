pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) {
        let status = if self.completed {
            "Выполнено"
        } else {
            "В процессе"
        };

        println!("[{}] {} — {}", self.id, self.title, status);
    }
}
