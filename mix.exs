defmodule Weld.MixProject do
  use Mix.Project

  @app :libweld
  @version "0.0.1"
  @native_app :weld_native

  def project do
    [
      app: @app,
      version: @version,
      elixir: "~> 1.9",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      rustler_crates: rustler_crates(),
      elixirc_paths: elixirc_paths(Mix.env()),
      package: package(),
      # description: description(),
      compilers: [:rustler] ++ Mix.compilers()
      # test_coverage: [tool: ExCoveralls],
      # docs: docs(),
      # preferred_cli_env: [
      #   coveralls: :test,
      #   "coveralls.detail": :test,
      #   "coveralls.post": :test,
      #   "coveralls.html": :test
      # ]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp rustler_crates() do
    [
      weld_native: [
        path: "native/weld_native",
        mode: rustc_mode(Mix.env())
      ]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp package do
    [
      maintainers: ["Dmitry Rubinstein"],
      licenses: ~w(MIT BSD-3-Clause),
      links: %{"Github" => "https://github.com/Virviil/libweld"},
      files: ~w(mix.exs lib) ++ rust_files()
    ]
  end

  defp rust_files do
    ~w(Cargo.toml src .cargo)
    |> Enum.map(&"native/#{@native_app}/#{&1}")
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.0"}
    ]
  end
end
