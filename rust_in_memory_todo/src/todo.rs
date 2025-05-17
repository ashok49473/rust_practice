// A struct to represent a single task
pub struct Task {
    pub title: String,
    pub completed: bool,
}

// Methods related to Task
impl Task {
    // Create a new task
    pub fn new(title: String) -> Self {
        Task {
            title,
            completed: false,
        }
    }

    // Mark the task as completed
    pub fn mark_done(&mut self) {
        self.completed = true;
    }
}
