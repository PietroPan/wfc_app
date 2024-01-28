defmodule WfcApp.Rust do
  use Rustler,
    otp_app: :wfc_app, # must match the name of the project in `mix.exs`
    crate: :wfc # must match the name of the crate in `native/rustlerpdf/Cargo.toml`

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def test_lib(), do: :erlang.nif_error(:nif_not_loaded)
end
