use kube::Client;

// The tokio::main procedural macro sets up the Tokio runtime which provides an I/O driver, task
// scheduler, timer, and blocking pool, necessary for running asynchronous tasks.
// Note that, the async function marked with this macro does not run as a worker. The expectation
// is that other tasks are spawned by the function here.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
  // This will first try to infer the kubeconfig. First, a user’s kubeconfig is loaded from
  // KUBECONFIG or ~/.kube/config. When that fails, an in-cluster config is loaded via
  // Config::incluster. When inference from both sources fails, then an error is returned.
  // Then it'll try to construct a Kube API server client using that inferred kubeconfig.
  let client = Client::try_default().await?;

  Ok(())
}
