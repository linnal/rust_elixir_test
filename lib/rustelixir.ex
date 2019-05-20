defmodule Math do
  use Rustler, otp_app: :rustelixir, crate: "math"

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)

  def sum_list(_arg1), do: :erlang.nif_error(:nif_not_loaded)

  def append_to_list(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
end
