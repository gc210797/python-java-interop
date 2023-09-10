use jni::objects::{JClass, JList, JString, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use libloading;
use numpy::PyArray2;
use pyo3::types::PyString;
use pyo3::{types::PyModule, PyResult, Python};

use pyo3::prelude::PyObject;

pub fn read_csv(url: String) -> PyResult<Vec<String>> {
    // Explanantion for manually loading dynlib is due to: https://github.com/PyO3/pyo3/issues/2000#issuecomment-977111582
    std::env::set_var("PYTHONUNBUFFERED", "1");
    let _ = unsafe {
        libloading::os::unix::Library::open(
            Some("/usr/lib64/libpython3.11.so"),
            libloading::os::unix::RTLD_NOW | libloading::os::unix::RTLD_GLOBAL,
        )
        .unwrap();
    };
    Python::with_gil(|py| {
        let my_mod = PyModule::import(py, "jep_test.example")?;
        let arr: &PyArray2<PyObject> = my_mod.getattr("slice_csv")?.call1((url,))?.extract()?;

        let readonly = arr.readonly();

        let mut values: Vec<String> = vec![];

        for x in readonly.as_slice()?.iter() {
            values.push(x.downcast::<PyString>(py)?.to_string());
        }

        Ok(values)
    })
}

#[no_mangle]
pub extern "system" fn Java_com_highradius_interop_App_readCsv<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jobject {
    let input: String = env
        .get_string(&input)
        .expect("unable to convert from java string")
        .into();
    let output = read_csv(input).unwrap();

    let obj = env
        .new_object(
            "java/util/ArrayList",
            "(I)V",
            &[JValue::Int(output.len() as i32)],
        )
        .unwrap();

    let list = JList::from_env(&mut env, &obj).unwrap();

    output.iter().for_each(|x| {
        let str = env.new_string(x).unwrap();

        let _ = list.add(&mut env, &str.into());
    });

    obj.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", read_csv("test".to_owned()));
        assert!(read_csv("test".to_owned()).is_ok());
    }
}
