fn main() -> anyhow::Result<()> {
  tonic_prost_build::configure().compile_protos(&["./spec/csi.proto"], &[])?;

  Ok(())
}
