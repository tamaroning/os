use super::arch::{ArchTask, ArchVm};
use crate::build_config::{NUM_TASK_MAX, TICK_HZ};
use alloc::collections::{BTreeMap, VecDeque};
use core::cell::UnsafeCell;

const TASK_QUANTUM: u64 = 20 * (TICK_HZ / 1000) as u64;

static mut TASKS: BTreeMap<TaskId, Task> = BTreeMap::new();
static mut RUN_QUEUE: VecDeque<TaskId> = VecDeque::new();

pub struct Task {
    inner: UnsafeCell<TaskInner>,
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

    //pub fn as_ref(&self) -> TaskRef {
    //    TaskRef { ref_: self as *mut Task }
    //}
}

impl Into<Task> for TaskInner {
    fn into(self) -> Task {
        Task::new(self)
    }
}

pub struct TaskRef {
    ref_: *mut Task,
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

#[derive(Debug, PartialEq)]
enum TaskState {
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
}

pub fn task_create(name: &'static str) -> &Task {
    let t = TaskInner::new(name);
    let tid = t.tid;
    unsafe {
        TASKS.insert(tid, Task::new(t));
        &TASKS.get(&tid).unwrap()
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
        RUN_QUEUE.push_back(task.get().tid);
    }
}

pub fn task_switch() {}
