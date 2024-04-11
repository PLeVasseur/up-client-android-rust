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
    pub async fn new(
        env: &JNIEnv<'_>,
        up_client: JObject<'_>,
        usub: JObject<'_>, /*, JNIEnv? */
    ) -> Self {
        // Convert local references to global references
        let up_client = env
            .new_global_ref(up_client)
            .expect("Failed to create global ref for up_client");
        let usub = env
            .new_global_ref(usub)
            .expect("Failed to create global ref for usub");

        // Obtain the JavaVM from the JNIEnv
        let vm = env.get_java_vm().expect("Failed to get JavaVM");

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
