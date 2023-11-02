defmodule SentencepieceTest do
  use ExUnit.Case
  doctest Sentencepiece

  test "greets the world" do
    assert Sentencepiece.hello() == :world
  end
end
