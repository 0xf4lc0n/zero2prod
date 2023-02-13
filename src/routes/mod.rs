mod health_check;
mod newsletter;
mod subscriptions;
mod subscriptions_confirmation;

pub use health_check::*;
pub use newsletter::*;
pub use subscriptions::*;
pub use subscriptions_confirmation::*;

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
