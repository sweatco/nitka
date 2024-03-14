use std::future::Future;

use futures::future::join_all;
use near_workspaces::types::Gas;
use tokio::spawn;

pub async fn scoped_command_measure<Input, Inputs, Command, Fut>(
    inputs: Inputs,
    mut command: Command,
) -> anyhow::Result<Vec<(Input, Gas)>>
where
    Input: Copy,
    Inputs: IntoIterator<Item = Input>,
    Fut: Future<Output = anyhow::Result<Gas>> + Send + 'static,
    Command: FnMut(Input) -> Fut + Copy,
{
    let inputs: Vec<_> = inputs.into_iter().collect();

    let all: Vec<_> = inputs
        .iter()
        .map(|inp| redundant_command_measure(move || command(*inp)))
        .collect();

    let res: Vec<_> = join_all(all).await.into_iter().collect::<anyhow::Result<_>>()?;

    let res: Vec<_> = inputs.into_iter().zip(res.into_iter()).collect();

    Ok(res)
}

/// This method runs the same command several times and checks if
/// all executions took the same anmount of gas
async fn redundant_command_measure<Fut>(mut command: impl FnMut() -> Fut) -> anyhow::Result<Gas>
where
    Fut: Future<Output = anyhow::Result<Gas>> + Send + 'static,
{
    let futures: Vec<_> = (0..1).map(|_| spawn(command())).collect();

    let all_gas: Vec<Gas> = join_all(futures)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect::<anyhow::Result<_>>()?;

    let gas = all_gas.first().unwrap();

    assert!(all_gas.iter().all(|g| g == gas));

    Ok(*gas)
}
