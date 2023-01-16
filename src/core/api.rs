#[derive(PartialEq, Debug)]
pub enum APIState {
    Uninitialized,
    OptionsBlock,
}

pub fn pbrt_init(current_state: &mut APIState) -> anyhow::Result<()> {
    if current_state != &APIState::Uninitialized {
        return Err(anyhow::anyhow!("pbrt_init() has already been called."));
    }

    *current_state = APIState::OptionsBlock;

    return Ok(());
}

pub fn pbrt_cleanup(current_state: &mut APIState) -> anyhow::Result<()> {
    if current_state == &APIState::Uninitialized {
        return Err(anyhow::anyhow!(
            "pbrt_cleanup() called without pbrt_init()."
        ));
    }

    *current_state = APIState::Uninitialized;
    return Ok(());
}
