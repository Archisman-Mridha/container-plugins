use {
  crate::csi::{identity_server::Identity, *},
  tonic::{Request, Response, Status}
};

/// Identity service RPCs allow a CO to query a plugin for capabilities, health, and other
/// metadata.
pub struct IdentityService {}

#[tonic::async_trait]
impl Identity for IdentityService {
  async fn get_plugin_info(
    &self,
    request: Request<GetPluginInfoRequest>
  ) -> Result<Response<GetPluginInfoResponse>, Status> {
    unimplemented!()
  }

  async fn get_plugin_capabilities(
    &self,
    request: Request<GetPluginCapabilitiesRequest>
  ) -> Result<Response<GetPluginCapabilitiesResponse>, Status> {
    unimplemented!()
  }

  async fn probe(&self, request: Request<ProbeRequest>) -> Result<Response<ProbeResponse>, Status> {
    unimplemented!()
  }
}
