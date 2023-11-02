defmodule SentencepieceTest do
  use ExUnit.Case
  # doctest Sentencepiece

  test "init model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    assert model != :nif_not_loaded
  end

  test "len model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    len = SentencePieceEx.len(model)
    assert len != :nif_not_loaded
  end

  test "bos_id model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    bos_id = SentencePieceEx.bos_id(model)
    assert bos_id != :nif_not_loaded
  end

  test "eos_id model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    eos_id = SentencePieceEx.eos_id(model)
    assert eos_id != :nif_not_loaded
  end

  test "unk_id model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    unk_id = SentencePieceEx.unk_id(model)
    assert unk_id != :nif_not_loaded
  end

  test "pad_id model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    pad_id = SentencePieceEx.pad_id(model)
    assert pad_id != :nif_not_loaded
  end

  test "decode_piece_ids model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    ids = [1, 2, 3]
    pieces = SentencePieceEx.decode_piece_ids(model, ids)
    assert pieces != :nif_not_loaded
  end

  test "encode sentence to ids model" do
    model_path = System.fetch_env!("MODEL_PATH")
    model = SentencePieceEx.init(model_path)
    sentence = "Hello world"
    ids = SentencePieceEx.encode_sentence_to_ids(model, sentence)
    assert ids != :nif_not_loaded
  end
end
