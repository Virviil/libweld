defmodule Weld.Native do
  use Rustler, otp_app: :libweld, crate: :weld_native

  def weld_module_compile(code, conf), do: error
  def weld_module_run(module, conf, input_value), do: error

  def weld_conf_new(), do: error
  def weld_conf_get(conf, key), do: error
  def weld_conf_set(conf, key, value), do: error

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
