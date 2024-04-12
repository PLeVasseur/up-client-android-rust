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
    uuri_class: GlobalRef,
    ustatus_class: GlobalRef,
    ulistener_native_bridge_class: GlobalRef,
    native_bridge_class: GlobalRef,
}

impl UPClientAndroid {
    pub async fn new(vm: JavaVM, up_client: GlobalRef, usub: GlobalRef, uuri_class: GlobalRef,
    ustatus_class: GlobalRef, ulistener_native_bridge_class: GlobalRef, native_bridge_class: GlobalRef) -> Self {
        trace!(
            "{}:{} Able to instantiate UPClientAndroid",
            UPCLIENTANDROID_TAG,
            UPCLIENTANDROID_FN_NEW_TAG
        );
        Self {
            vm,
            up_client,
            usub,
            uuri_class,
            ustatus_class,
            ulistener_native_bridge_class,
            native_bridge_class
        }
    }
}
