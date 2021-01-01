use crate::{CommandQueueOptions, Dims, KernelArg, KernelArgPtr, KernelError, Output, Work};

#[derive(Debug)]
pub struct KernelOperation<'a> {
    _name: String,
    _args: Vec<KernelArg<'a>>,
    _work: Option<Work>,
    pub command_queue_opts: Option<CommandQueueOptions>,
}

impl<'a> KernelOperation<'a> {
    pub fn new(name: &str) -> KernelOperation<'a> {
        KernelOperation {
            _name: name.to_owned(),
            _args: vec![],
            _work: None,
            command_queue_opts: None,
        }
    }

    pub fn name(&self) -> &str {
        &self._name[..]
    }

    pub fn command_queue_opts(&self) -> Option<CommandQueueOptions> {
        self.command_queue_opts.clone()
    }

    pub fn args(&self) -> &[KernelArg<'a>] {
        &self._args[..]
    }

    pub fn mut_args(&mut self) -> &mut [KernelArg<'a>] {
        &mut self._args[..]
    }

    pub fn with_dims<D: Into<Dims>>(mut self, dims: D) -> KernelOperation<'a> {
        self._work = Some(Work::new(dims.into()));
        self
    }

    pub fn with_work<W: Into<Work>>(mut self, work: W) -> KernelOperation<'a> {
        self._work = Some(work.into());
        self
    }

    pub fn add_arg<A: KernelArgPtr>(mut self, arg: &'a A) -> KernelOperation<'a> {
        self._args.push(KernelArg::new(arg));
        self
    }

    pub fn with_command_queue_options(mut self, opts: CommandQueueOptions) -> KernelOperation<'a> {
        self.command_queue_opts = Some(opts);
        self
    }

    pub fn argc(&self) -> usize {
        self._args.len()
    }

    #[inline]
    pub fn work(&self) -> Output<Work> {
        self._work
            .clone()
            .ok_or_else(|| KernelError::WorkIsRequired.into())
    }
}
