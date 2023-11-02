defmodule SentencePieceEx do
  @moduledoc false

  use Rustler, otp_app: :sentencepiece, crate: :sentencepieceex

  def init(_model_path), do: :erlang.nif_error(:nif_not_loaded)
  def len(_model), do: :erlang.nif_error(:nif_not_loaded)

  @spec bos_id(any()) :: any()
  def bos_id(_model), do: :erlang.nif_error(:nif_not_loaded)
  def eos_id(_model), do: :erlang.nif_error(:nif_not_loaded)
  def unk_id(_model), do: :erlang.nif_error(:nif_not_loaded)
  def pad_id(_model), do: :erlang.nif_error(:nif_not_loaded)

  def decode_piece_ids(_model, _ids), do: :erlang.nif_error(:nif_not_loaded)
  def decode_pieces(_model, _pieces), do: :erlang.nif_error(:nif_not_loaded)
  def encode(_model, _sentence), do: :erlang.nif_error(:nif_not_loaded)
  def encode_sentence_to_ids(_model, _sentence), do: :erlang.nif_error(:nif_not_loaded)

  def is_empty(_model), do: :erlang.nif_error(:nif_not_loaded)
  def piece_to_id(_model, _piece), do: :erlang.nif_error(:nif_not_loaded)

  def sample_encode(_model, _sentence, _nbest_size, _alpha),
    do: :erlang.nif_error(:nif_not_loaded)

  def get_piece_from_piece_with_id_model(_model), do: :erlang.nif_error(:nif_not_loaded)
  def get_id_from_piece_with_id_model(_model), do: :erlang.nif_error(:nif_not_loaded)
  def get_span_from_piece_with_id_model(_model), do: :erlang.nif_error(:nif_not_loaded)
end
