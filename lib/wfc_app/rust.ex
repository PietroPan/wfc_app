defmodule WfcApp.Rust do
  use Rustler,
    otp_app: :wfc_app, # must match the name of the project in `mix.exs`
    crate: :wfc # must match the name of the crate in `native/rustlerpdf/Cargo.toml`

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
  def generate_image(_rule_set, _tile_set, _symmetry, _size, _results, _name), do: :erlang.nif_error(:nif_not_loaded)
end
