use core::cell::UnsafeCell;

use alloc::vec::Vec;

use super::arch::{ArchTask, ArchVm};

const NUM_TASK_MAX: usize = 16;

static mut TASKS: Vec<Task> = Vec::new();
static mut RUN_QUEUE: Vec<TaskId> = Vec::new();

pub struct Task {
    inner: UnsafeCell<TaskInner>,
}

impl Default for Task {
    fn default() -> Task {
        Task::new(TaskInner::unused())
    }
}

impl Task {
    pub const fn new(task: TaskInner) -> Task {
        Task {
            inner: UnsafeCell::new(task),
        }
    }

    pub fn get(&self) -> &TaskInner {
        unsafe { &*self.inner.get() }
    }

    pub fn get_mut(&self) -> &mut TaskInner {
        unsafe { &mut *self.inner.get() }
    }
}

impl Into<Task> for TaskInner {
    fn into(self) -> Task {
        Task::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId {
    private: usize,
}

impl TaskId {
    fn new() -> TaskId {
        for i in 0..NUM_TASK_MAX {
            if unsafe { TASKS[i].get_mut().state == TaskState::Unused } {
                return TaskId { private: i };
            }
        }
        panic!("no more task slots")
    }
}

#[derive(Debug, PartialEq)]
enum TaskState {
    Unused,
    Runnable,
    Blocked,
}

pub struct TaskInner {
    arch: ArchTask,
    vm: ArchVm,
    tid: TaskId,
    name: &'static str,
    state: TaskState,
    destroyed: bool,
    //pager: TaskId, // pager task
    //timeout: u64,
    //ref_count: u64,
    //quantum: u64,
}

impl TaskInner {
    fn new(name: &'static str) -> TaskInner {
        TaskInner {
            arch: ArchTask {},
            vm: ArchVm {},
            tid: TaskId::new(),
            name,
            state: TaskState::Runnable,
            destroyed: false,
        }
    }

    const fn unused() -> TaskInner {
        TaskInner {
            arch: ArchTask {},
            vm: ArchVm {},
            tid: TaskId { private: 0 },
            name: "",
            state: TaskState::Unused,
            destroyed: false,
        }
    }
}

pub fn task_create(name: &'static str) -> &Task {
    let t = TaskInner::new(name);
    let tid = t.tid;
    unsafe {
        TASKS[tid.private] = Task::new(t);
        &TASKS[tid.private]
    }
}

pub fn task_block(task: &Task) {
    debug_assert!(task.get().state == TaskState::Runnable);
    task.get_mut().state = TaskState::Blocked;
}

pub fn task_resume(task: &Task) {
    debug_assert!(task.get().state == TaskState::Blocked);
    task.get_mut().state = TaskState::Runnable;
    unsafe {
        // HACK: break borrowing rules
        RUN_QUEUE.push(task.get().tid);
    }
}
