mod batch_loader;

pub use batch_loader::load_programs;
pub use batch_loader::jump_to_next_program;
pub use batch_loader::INSTANCE_STACK_BASE;
pub use batch_loader::switch_to_instance_stack;
