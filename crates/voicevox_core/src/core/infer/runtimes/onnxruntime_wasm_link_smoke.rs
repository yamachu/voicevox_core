use std::sync::Arc;

use anyhow::{anyhow, bail};
use ndarray::{Array, Dimension};

use super::blocking::Onnxruntime;
use crate::core::devices::{GpuSpec, SupportedDevices};
use crate::core::{
    infer::{
        InferenceRuntime, InferenceSessionOptions, InputScalarKind, OutputScalarKind, OutputTensor,
        ParamInfo, PushInputTensor,
    },
    voice_model::ModelBytes,
};

pub(crate) struct RunContext;

impl From<Arc<()>> for RunContext {
    fn from(_: Arc<()>) -> Self {
        Self
    }
}

impl PushInputTensor for RunContext {
    fn push_int64(
        &mut self,
        _: &'static str,
        _: Array<i64, impl Dimension + 'static>,
    ) -> anyhow::Result<()> {
        bail!("WASM link-smoke runtime cannot run inference")
    }

    fn push_float32(
        &mut self,
        _: &'static str,
        _: Array<f32, impl Dimension + 'static>,
    ) -> anyhow::Result<()> {
        bail!("WASM link-smoke runtime cannot run inference")
    }
}

impl InferenceRuntime for Onnxruntime {
    type Session = ();
    type RunContext = RunContext;

    const DISPLAY_NAME: &'static str = "WASM link-smoke runtime";

    fn supported_devices(&self) -> crate::Result<SupportedDevices> {
        Err(anyhow!("WASM link-smoke runtime does not support inference").into())
    }

    fn test_gpu(&self, _: GpuSpec) -> anyhow::Result<()> {
        bail!("WASM link-smoke runtime does not support GPU inference")
    }

    fn new_session(
        &self,
        _: &ModelBytes,
        _: InferenceSessionOptions,
    ) -> anyhow::Result<(
        Self::Session,
        Vec<ParamInfo<InputScalarKind>>,
        Vec<ParamInfo<OutputScalarKind>>,
    )> {
        bail!("WASM link-smoke runtime cannot create inference sessions")
    }

    fn run_blocking(_: Self::RunContext) -> anyhow::Result<Vec<OutputTensor>> {
        bail!("WASM link-smoke runtime cannot run inference")
    }

    async fn run_async(_: Self::RunContext, _: bool) -> anyhow::Result<Vec<OutputTensor>> {
        bail!("WASM link-smoke runtime cannot run inference")
    }
}
