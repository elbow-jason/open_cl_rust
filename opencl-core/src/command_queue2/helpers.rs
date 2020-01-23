use crate::event::wait_list::WaitList;

pub struct CommandQueueOptions {
    pub is_blocking: bool,
    pub offset: usize,
    pub wait_list: WaitList,
}

impl Default for CommandQueueOptions {
    fn default() -> CommandQueueOptions {
        CommandQueueOptions {
            is_blocking: true,
            offset: 0,
            wait_list: WaitList::empty(),
        }
    }
}
