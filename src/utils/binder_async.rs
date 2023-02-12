/*
 * Copyright (C) 2021 The Android Open Source Project
 * Copyright (C) 2023 GloDroid project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use async_std::task::{block_on, spawn_blocking};
use binder::binder_impl::BinderAsyncRuntime;
use binder::{BinderAsyncPool, BoxFuture, StatusCode};
use std::future::Future;

pub enum Async {}

impl BinderAsyncPool for Async {
    fn spawn<'a, F1, F2, Fut, A, B, E>(spawn_me: F1, after_spawn: F2) -> BoxFuture<'a, Result<B, E>>
    where
        F1: FnOnce() -> A,
        F2: FnOnce(A) -> Fut,
        Fut: Future<Output = Result<B, E>>,
        F1: Send + 'static,
        F2: Send + 'a,
        Fut: Send + 'a,
        A: Send + 'static,
        B: Send + 'a,
        E: From<StatusCode>,
    {
        if binder::is_handling_transaction() {
            // We are currently on the thread pool for a binder server, so we should execute the
            // transaction on the current thread so that the binder kernel driver is able to apply
            // its deadlock prevention strategy to the sub-call.
            //
            // This shouldn't cause issues with blocking the thread as only one task will run in a
            // call to `block_on`, so there aren't other tasks to block.
            let result = spawn_me();
            Box::pin(after_spawn(result))
        } else {
            let handle = spawn_blocking(spawn_me);
            Box::pin(async move {
                // The `is_panic` branch is not actually reachable in Android as we compile
                // with `panic = abort`.
                let res = handle.await;
                after_spawn(res).await
            })
        }
    }
}

/// Wrapper around Tokio runtime types for providing a runtime to a binder server.
pub struct AsyncStdRuntime;

impl BinderAsyncRuntime for AsyncStdRuntime {
    fn block_on<F: Future>(&self, future: F) -> F::Output {
        block_on(future)
    }
}
