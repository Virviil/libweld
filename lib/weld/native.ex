defmodule Weld.Native do
  use Rustler, otp_app: :libweld, crate: :weld_native

  @type weld_config :: reference()
  @type weld_module :: reference()
  @type weld_value :: reference()

  @spec weld_module_compile(code :: String.t(), config :: weld_config()) ::
          {:ok, module :: weld_module()} | {:error, reason :: String.t()}
  def weld_module_compile(_code, _config), do: error()

  @spec weld_module_run(
          module :: weld_module(),
          config :: weld_config(),
          input_value :: weld_value()
        ) :: {:ok, result :: weld_value()} | {:error, reason :: String.t()}
  def weld_module_run(_module, _config, _input_value), do: error()

  @spec weld_conf_new :: weld_config()
  def weld_conf_new(), do: error()
  @spec weld_conf_get(config :: weld_config(), key :: String.t()) :: nil | String.t()
  def weld_conf_get(_config, _key), do: error()
  @spec weld_conf_set(config :: weld_config(), key :: String.t(), value :: String.t()) :: :ok
  def weld_conf_set(_config, _key, _value), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
