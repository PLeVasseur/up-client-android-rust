use jni::objects::{GlobalRef, JObject};
use jni::{JNIEnv, JavaVM};

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

        Self {
            vm,
            up_client,
            usub,
        }
    }
}
