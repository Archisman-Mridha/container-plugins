# Writing a Container Runtime from scratch (in Rust ofc!)

## GOTCHAs

- runc reports the container creation error via its stderr. Since exactly the same file descriptor, later on, can become the container process' stderr the shim needs to carefully consume all the data in it until the runtime process termination occurs and report it to the container manager immediately.

  Thus, if we make a typo in our container command, the actual error will be reported back to us during the container creation phase:

  ```sh
  docker run -it ubuntu bahs

  docker: Error response from daemon: OCI runtime create failed: container_linux.go:345: starting container process caused "exec: \"bahs\": executable file not found in $PATH": unknown
  ```

- Linux doesn't allow zombie processes. In containers, one process must be the init process for each PID namespace.
  With Docker, each container usually has its own PID namespace and the `ENTRYPOINT` process is the init process.

## TODOs

- [ ] Understand about `subreapers` by following [Dealing with process termination in Linux (with Rust examples)](https://iximiuz.com/en/posts/dealing-with-processes-termination-in-Linux/).

- [ ] Read in detail about `supervisord`.

- [ ] Read about [User Namespaces](https://kubernetes.io/docs/concepts/workloads/pods/user-namespaces/#)

## REFERENCEs

- [Understanding the image layers](https://docs.docker.com/get-started/docker-concepts/building-images/understanding-image-layers/)

- [Container Runtime Interface (CRI): Past, Present, and Future](https://www.aquasec.com/cloud-native-academy/container-security/container-runtime-interface/)

- [Implementing Container Manager](https://iximiuz.com/en/series/implementing-container-manager/)

  > Incredible image summarizing Docker's layered architecture : https://iximiuz.com/conman-the-container-manager-inception/docker-containerd-runc-2000-opt.png.

- [The Almighty Pause Container](https://www.ianlewis.org/en/almighty-pause-container)

- [runc : CLI tool for spawning and running containers according to the OCI specification](https://github.com/opencontainers/runc)
- [containerd : An open and reliable container runtime](https://github.com/containerd/containerd)

- [How to Configure AppArmor for Security on Debian or Ubuntu](https://jumpcloud.com/blog/how-to-configure-apparmor-for-security-debian-ubuntu)

- [Kubernetes + Encrypted Memory = Security \* Privacy - Harshal Patil & Pradipta Banerjee, IBM](https://www.youtube.com/watch?v=pGMdSFlD0_E)
- [From Sandboxed Containers to Confidential Containers — Part-1](https://pradiptabanerjee.medium.com/from-sandboxed-containers-to-confidential-containers-enabling-cloud-native-confidential-computing-35936fad5998)

## TOOLs

- [Buildah : a tool that facilitates building Open Container Initiative (OCI) container images](https://github.com/containers/buildah)
