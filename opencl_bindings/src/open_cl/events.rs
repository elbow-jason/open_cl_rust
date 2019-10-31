use crate::ffi::{
    cl_command_queue,
    cl_context,
    cl_command_type,
    cl_int,
    cl_uint,
    cl_event_info,
    clGetEventInfo,        
};

use crate::open_cl::{ClObject, into_result, Output};
use crate::command_queue::CommandQueue;
use crate::context::Context;
use crate::event::Event;

/* command execution status */
crate::__codes_enum!(CommandExecutionStatus, cl_int, {
    Complete => 0x0,
    Running => 0x1,
    Submitted => 0x2,
    Queued => 0x3
});

// The cl_command_type is the return type of clGetEventInfo.
crate::__codes_enum!(CommandType, cl_command_type, {
    NdrangeKernel => 0x11F0,
    Task => 0x11F1,
    NativeKernel => 0x11F2,
    ReadBuffer => 0x11F3,
    WriteBuffer => 0x11F4,
    CopyBuffer => 0x11F5,
    ReadImage => 0x11F6,
    WriteImage => 0x11F7,
    CopyImage => 0x11F8,
    CopyImageToBuffer => 0x11F9,
    CopyBufferToImage => 0x11FA,
    MapBuffer => 0x11FB,
    MapImage => 0x11FC,
    UnmapMemObject => 0x11FD,
    Marker => 0x11FE,
    AcquireGlObjects => 0x11FF,
    ReleaseGlObjects => 0x1200,
    ReadBufferRect => 0x1201,
    WriteBufferRect => 0x1202,
    CopyBufferRect => 0x1203,
    User => 0x1204,
    Barrier => 0x1205,
    MigrateMemObjects => 0x1206,
    FillBuffer => 0x1207,
    FillImage => 0x1208,
    SvmFree => 0x1209,
    SvmMemcpy => 0x120A,
    SvmMemfill => 0x120B,
    SvmMap => 0x120C,
    SvmUnmap => 0x120D
});


/* cl_event_info */
crate::__codes_enum!(EventInfoFlag, cl_event_info, {
    CommandQueue => 0x11D0,
    CommandType => 0x11D1,
    ReferenceCount => 0x11D2,
    CommandExecutionStatus => 0x11D3,
    Context => 0x11D4
});

impl EventInfoFlag {
    fn return_size_t(&self) -> libc::size_t {
        use EventInfoFlag as F;
        match self {
            F::CommandQueue => size_t!(cl_command_queue),
            F::Context => size_t!(cl_context),
            F::CommandType => size_t!(cl_command_type),
            F::CommandExecutionStatus => size_t!(cl_int),
            F::ReferenceCount => size_t!(cl_uint),
        }
    }
}

#[derive(Debug)]
pub enum EventInfo {
    CommandQueue(CommandQueue),
    Context(Context),
    // NOTE: add support for khr.
    CommandType(CommandType),
    CommandExecutionStatus(CommandExecutionStatus),
    ReferenceCount(usize)
}

impl EventInfo {
    unsafe fn from_raw_parts(
        info_flag: EventInfoFlag,
        return_value: usize,
    ) -> EventInfo {
        use EventInfoFlag as F;

        match info_flag {
            F::CommandQueue => {
                let command_queue: CommandQueue = CommandQueue::new(return_value as cl_command_queue);
                EventInfo::CommandQueue(command_queue)
            },
            F::CommandType => {
                let command_type: CommandType = CommandType::from(return_value as cl_command_type);
                EventInfo::CommandType(command_type)
            },
            F::ReferenceCount => EventInfo::ReferenceCount(return_value as usize),
            F::CommandExecutionStatus => {
                let ce_status = CommandExecutionStatus::from(return_value as cl_int);
                EventInfo::CommandExecutionStatus(ce_status)
            }
            F::Context => {
                let context: Context = Context::new(return_value as cl_context);
                EventInfo::Context(context)
            }
        }
    }   
}

pub fn cl_get_event_info(event: &Event, info_flag: EventInfoFlag) -> Output<EventInfo> {
    let expected_size_of_return = info_flag.return_size_t();
    let return_value_size = 0usize;
    let return_value = 0usize;

    let err_code = unsafe {    
        clGetEventInfo (
            event.raw_cl_object(),
            info_flag as cl_event_info,
            expected_size_of_return,
            return_value as *mut libc::c_void,
            return_value_size as *mut usize,
        )
    };
    let () = into_result(err_code, ())?;

    let event_info = unsafe { 
        EventInfo::from_raw_parts(
            info_flag,
            return_value,
            // return_value_size,
            // expected_size_of_return
        )
    };
    Ok(event_info)

}

