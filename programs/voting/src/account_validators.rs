use crate::*;

pub trait Validate<'info> {
    fn validate(&self) -> Result<()>;
}

impl<'info> Validate<'info> for ActivatePoll<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
