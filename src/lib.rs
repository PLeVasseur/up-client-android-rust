pub mod utransport;

use jni::objects::{GlobalRef, JObject};
use jni::{JNIEnv, JavaVM};
use log::trace;

const UPCLIENTANDROID_TAG: &str = "UPClientAndroid:";
const UPCLIENTANDROID_FN_NEW_TAG: &str = "new():";

pub struct UPClientAndroid {
    vm: JavaVM,
    up_client: GlobalRef,
    usub: GlobalRef,
}

impl UPClientAndroid {
    pub async fn new(vm: JavaVM, up_client: GlobalRef, usub: GlobalRef) -> Self {
        trace!(
            "{}:{} Able to convert local refs to global ones and obtain a vm",
            UPCLIENTANDROID_TAG,
            UPCLIENTANDROID_FN_NEW_TAG
        );

        Self {
            vm,
            up_client,
            usub,
        }
    }
}
