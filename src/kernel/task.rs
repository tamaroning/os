use super::arch::{ArchTask, ArchVm};
use crate::build_config::{NUM_TASK_MAX, TICK_HZ};
use alloc::collections::{BTreeMap, VecDeque};
use core::ops::{Deref, DerefMut};

const TASK_QUANTUM: u64 = 20 * (TICK_HZ / 1000) as u64;

static mut TASKS: BTreeMap<TaskId, Task> = BTreeMap::new();
static mut RUN_QUEUE: VecDeque<TaskRef> = VecDeque::new();

#[derive(Debug, Clone, Copy)]
pub struct Task {
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

impl Task {
    fn new(name: &'static str) -> Task {
        Task {
            arch: ArchTask {},
            vm: ArchVm {},
            tid: TaskId::new(),
            name,
            state: TaskState::Runnable,
            destroyed: false,
        }
    }

    pub const fn dummy() -> Task {
        Task {
            arch: ArchTask {},
            vm: ArchVm {},
            tid: TaskId { private: 0 },
            name: "dummy",
            state: TaskState::Runnable,
            destroyed: false,
        }
    }

    pub const fn as_ref(&self) -> TaskRef {
        TaskRef {
            ref_: self as *const Task as *mut Task,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TaskRef {
    ref_: *mut Task,
}

impl Deref for TaskRef {
    type Target = Task;

    fn deref(&self) -> &Task {
        unsafe { &*self.ref_ }
    }
}

impl DerefMut for TaskRef {
    fn deref_mut(&mut self) -> &mut Task {
        unsafe { &mut *self.ref_ }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId {
    private: usize,
}

impl TaskId {
    fn new() -> TaskId {
        for i in 0..NUM_TASK_MAX {
            unsafe {
                if TASKS.contains_key(&TaskId { private: i }) {
                    continue;
                }
                return TaskId { private: i };
            }
        }
        panic!("no more task slots")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TaskState {
    Runnable,
    Blocked,
}

pub fn task_create(name: &'static str) -> TaskRef {
    let t = Task::new(name);
    let tid = t.tid;
    unsafe {
        TASKS.insert(tid, t);
        TASKS.get(&tid).unwrap().as_ref()
    }
}

pub fn task_block(task: &mut Task) {
    debug_assert!(task.state == TaskState::Runnable);
    task.state = TaskState::Blocked;
}

pub fn task_resume(task: &mut Task) {
    debug_assert!(task.state == TaskState::Blocked);
    task.state = TaskState::Runnable;
    unsafe {
        RUN_QUEUE.push_back(task.as_ref());
    }
}

pub fn task_switch() {}

pub fn task_init_per_cpu() {
    // https://github.com/nuta/microkernel-book/blob/2a49c4a932208ae22c0727cdd2047bf277bf447b/kernel/task.c#L322
}
