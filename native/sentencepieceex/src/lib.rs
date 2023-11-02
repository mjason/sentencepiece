use rustler::{Env, NifResult, ResourceArc};
use sentencepiece::PieceWithId;
use sentencepiece::SentencePieceError;
use sentencepiece::SentencePieceProcessor;
use std::sync::Arc;

pub struct SentencePieceModel {
    pub sp: Arc<SentencePieceProcessor>,
}
unsafe impl Sync for SentencePieceModel {}

pub struct PieceWithIdModel(PieceWithId);

fn init_model(model_path: String) -> Result<SentencePieceModel, SentencePieceError> {
    let sp = SentencePieceProcessor::open(model_path)?;

    Ok(SentencePieceModel { sp: Arc::new(sp) })
}

#[rustler::nif(schedule = "DirtyIo")]
fn init(model_path: String) -> NifResult<ResourceArc<SentencePieceModel>> {
    let model =
        init_model(model_path).map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

    Ok(ResourceArc::new(model))
}

#[rustler::nif]
fn len(model: ResourceArc<SentencePieceModel>) -> NifResult<usize> {
    let sp = model.sp.to_owned();
    Ok(sp.len())
}

#[rustler::nif]
fn bos_id(model: ResourceArc<SentencePieceModel>) -> NifResult<u32> {
    let sp = model.sp.to_owned();
    Ok(sp.bos_id().unwrap_or(0))
}

#[rustler::nif]
fn eos_id(model: ResourceArc<SentencePieceModel>) -> NifResult<u32> {
    let sp = model.sp.to_owned();
    Ok(sp.eos_id().unwrap_or(0))
}

#[rustler::nif]
fn pad_id(model: ResourceArc<SentencePieceModel>) -> NifResult<u32> {
    let sp = model.sp.to_owned();
    Ok(sp.pad_id().unwrap_or(0))
}

#[rustler::nif]
fn unk_id(model: ResourceArc<SentencePieceModel>) -> NifResult<u32> {
    let sp = model.sp.to_owned();
    Ok(sp.unk_id())
}

#[rustler::nif(schedule = "DirtyIo")]
fn decode_piece_ids(model: ResourceArc<SentencePieceModel>, ids: Vec<u32>) -> NifResult<String> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();
    let decoded = sp
        .decode_piece_ids(&ids)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
    Ok(decoded)
}

#[rustler::nif(schedule = "DirtyIo")]
fn decode_pieces(model: ResourceArc<SentencePieceModel>, pieces: Vec<String>) -> NifResult<String> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();
    let decoded = sp
        .decode_pieces(&pieces)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
    Ok(decoded)
}

#[rustler::nif(schedule = "DirtyIo")]
fn encode(
    model: ResourceArc<SentencePieceModel>,
    sentence: String,
) -> NifResult<Vec<ResourceArc<PieceWithIdModel>>> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();

    let encoded = sp
        .encode(&sentence)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

    Ok(encoded
        .into_iter()
        .map(|piece| ResourceArc::new(PieceWithIdModel(piece)))
        .collect())
}

#[rustler::nif(schedule = "DirtyIo")]
fn encode_sentence_to_ids(
    model: ResourceArc<SentencePieceModel>,
    sentence: String,
) -> NifResult<Vec<u32>> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();

    let encoded = sp
        .encode(&sentence)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

    let ids: Vec<u32> = encoded.into_iter().map(|piece| piece.id).collect();

    Ok(ids)
}

#[rustler::nif]
fn is_empty(model: ResourceArc<SentencePieceModel>) -> NifResult<bool> {
    let sp = model.sp.to_owned();
    Ok(sp.is_empty())
}

#[rustler::nif]
fn piece_to_id(model: ResourceArc<SentencePieceModel>, piece: String) -> NifResult<u32> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();
    let id = sp
        .piece_to_id(&piece)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?
        .unwrap_or(0);
    Ok(id)
}

#[rustler::nif(schedule = "DirtyIo")]
fn sample_encode(
    model: ResourceArc<SentencePieceModel>,
    sentence: String,
    n_best: usize,
    alpha: f32,
) -> NifResult<Vec<ResourceArc<PieceWithIdModel>>> {
    let sp: Arc<SentencePieceProcessor> = model.sp.to_owned();

    let encoded = sp
        .sample_encode(&sentence, n_best, alpha)
        .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

    Ok(encoded
        .into_iter()
        .map(|piece| ResourceArc::new(PieceWithIdModel(piece)))
        .collect())
}

// PieceWithIdModel functions

#[rustler::nif]
fn get_piece_from_piece_with_id_model(model: ResourceArc<PieceWithIdModel>) -> NifResult<String> {
    let piece: String = model.0.piece.clone();
    Ok(piece)
}

#[rustler::nif]
fn get_id_from_piece_with_id_model(model: ResourceArc<PieceWithIdModel>) -> NifResult<u32> {
    let id: u32 = model.0.id.clone();
    Ok(id)
}

#[rustler::nif]
fn get_span_from_piece_with_id_model(
    model: ResourceArc<PieceWithIdModel>,
) -> NifResult<(u32, u32)> {
    let span: (u32, u32) = model.0.span.clone();
    Ok(span)
}

rustler::init!(
    "Elixir.SentencePieceEx",
    [
        init,
        len,
        bos_id,
        eos_id,
        pad_id,
        unk_id,
        decode_piece_ids,
        decode_pieces,
        encode,
        is_empty,
        piece_to_id,
        sample_encode,
        get_piece_from_piece_with_id_model,
        get_id_from_piece_with_id_model,
        get_span_from_piece_with_id_model,
        encode_sentence_to_ids
    ],
    load = |env: Env, _| {
        rustler::resource!(SentencePieceModel, env);
        rustler::resource!(PieceWithIdModel, env);
        true
    }
);
