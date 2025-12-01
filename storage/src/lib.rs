mod csi {
  //! The Container Storage Interface (CSI) is a standard for exposing arbitrary block and file
  //! storage systems to containerized workloads on Container Orchestration Systems (COs) like
  //! Kubernetes. Using CSI third-party storage providers can write and deploy plugins exposing new
  //! storage systems in Kubernetes without ever having to touch the core Kubernetes code.

  tonic::include_proto!("csi.v1");
}

pub mod services;
