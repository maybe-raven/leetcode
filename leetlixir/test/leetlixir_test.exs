defmodule LeetlixirTest do
  use ExUnit.Case
  doctest Leetlixir

  test "greets the world" do
    assert Leetlixir.hello() == :world
  end
end
