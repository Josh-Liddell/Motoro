use jlrs::prelude::*;

// Here is for tasks you can dispatch to the julia runtime

#[derive(serde::Deserialize)]
pub struct AdditionTask {
    pub a: f64,
    pub b: f64,
}

impl AsyncTask for AdditionTask {
    type Output = f64;

    async fn run<'frame>(self, mut frame: AsyncGcFrame<'frame>) -> Self::Output {
        let v1 = Value::new(&mut frame, self.a);
        let v2 = Value::new(&mut frame, self.b);
        let add_fn = Module::base(&frame)
            .global(&mut frame, "+")
            .expect("cannot find Base.+");

        // Safety: just adding two floating-point numbers
        unsafe { add_fn.call_async(&mut frame, [v1, v2]) }
            .await
            .expect("caught an exception")
            .unbox::<f64>()
            .expect("cannot unbox as f64")
    }
}
