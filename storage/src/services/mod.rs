//! Kubernetes is as minimally prescriptive about packaging and deployment of a CSI Volume Driver
//! as possible. The only requirements are around how Kubernetes (master and node) components find
//! and communicate with a CSI driver. Specifically, the following is dictated by Kubernetes
//! regarding CSI:
//!
//!   (1) Kubelet to CSI Driver Communication :
//!
//!     Kubelet discovers our CSI driver (and the Unix Domain Socket to use to interact with a CSI
//!     driver) via the kubelet plugin registration mechanism. It then directly issues CSI calls
//!     (like NodeStageVolume, NodePublishVolume, etc.) to CSI drivers via the Unix Domain Socket
//!     to mount and unmount volumes.
//!
//!   (2) Master to CSI Driver Communication :
//!
//!     Kubernetes master components do not communicate directly (via a Unix Domain Socket or
//!     otherwise) with CSI drivers. They interact only with the Kubernetes API.
//!
//!     Kubernetes CSI Sidecar Containers are a set of standard containers that aim to simplify the
//!     development and deployment of CSI Drivers on Kubernetes. These containers contain common
//!     logic to watch the Kubernetes API, trigger appropriate operations against the “CSI volume
//!     driver” container, and update the Kubernetes API as appropriate.
//!     They will be bundled with our CSI driver container, being deployed in the same pod.
//!
//! Our CSI driver thus only needs to implement the Identity, Node, and Controller services
//! described in the CSI specification.

pub mod controller;
pub mod identity;
pub mod node;
