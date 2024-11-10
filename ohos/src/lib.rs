use std::collections::HashMap;
use mobile::MobileConfig as RMobileConfig;
use napi_derive_ohos::napi;
use napi_ohos::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Error, Result, Status, JsFunction
};
use napi_ohos::bindgen_prelude;
use napi_ohos::threadsafe_function::ErrorStrategy;
use napi_ohos::threadsafe_function::ThreadSafeCallContext;


// / use napi::{
// /     threadsafe_function::{
// /         ThreadSafeCallContext, ThreadsafeFunctionCallMode, ThreadsafeFunctionReleaseMode,
// /     },

#[napi(object)]
pub struct MobileConfig {
    pub id: String,
    pub username: String,
    pub pwd: String,
    pub user_properties: Option<HashMap<String, String>>,
}

fn mq_config_2_inner(config: MobileConfig) -> RMobileConfig {
    RMobileConfig {
        id: config.id,
        username: config.username,
        pwd: config.pwd,
        user_properties: config.user_properties,
    }
}

struct FfiLog {
    logger: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled>,
}

impl mobile::Log for FfiLog {
    fn log(&self, msg: &str) {
        self.logger
            .call(Ok(msg.to_string()), ThreadsafeFunctionCallMode::Blocking);
    }
}

#[napi(ts_args_type = "callback: (msg: string) => void")]
pub fn set_logger(callback: JsFunction) -> Result<()> {
    let call_fn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
        .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
            ctx.env.create_string(ctx.value.as_str()).map(|v| vec![v])
        })?;
        mobile::set_logger(Box::new(FfiLog { logger: call_fn }))
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
}

#[napi]
pub fn hello_rust(config: MobileConfig) -> Result<()> {
    mobile::hello_rust(mq_config_2_inner(config))
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
