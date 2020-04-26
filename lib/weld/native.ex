defmodule Weld.Native do
  use Rustler, otp_app: :libweld, crate: :weld_native

  def add(a, b), do: error

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
