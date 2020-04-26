defmodule WeldTest do
  use ExUnit.Case
  doctest Weld

  test "greets the world" do
    assert Weld.hello() == :world
  end
end
