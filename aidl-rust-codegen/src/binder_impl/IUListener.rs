/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iulistener-rust-source/gen/org/eclipse/uprotocol/core/ubus/IUListener.rs.d -o out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iulistener-rust-source/gen -Nexternal/rust/up-client-android-rust/aidl-rust-codegen/src/aidl external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org/eclipse/uprotocol/core/ubus/IUListener.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
use crate::parcelable_stubs;
declare_binder_interface! {
  IUListener["org.eclipse.uprotocol.core.ubus.IUListener"] {
    native: BnUListener(on_transact),
    proxy: BpUListener {
    },
    async: IUListenerAsync,
  }
}
pub trait IUListener: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUListener" }
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()>;
  fn getDefaultImpl() -> IUListenerDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IUListenerDefaultRef) -> IUListenerDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IUListenerAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUListener" }
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> std::future::Ready<binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IUListenerAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUListener" }
  async fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()>;
}
impl BnUListener {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IUListener>
  where
    T: IUListenerAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IUListener for Wrapper<T, R>
    where
      T: IUListenerAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onReceive(_arg_event))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IUListenerDefault: Send + Sync {
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#onReceive: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IUListenerDefaultRef = Option<std::sync::Arc<dyn IUListenerDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IUListenerDefaultRef> = std::sync::Mutex::new(None);
impl BpUListener {
  fn build_parcel_onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_event)?;
    Ok(aidl_data)
  }
  fn read_response_onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUListener>::getDefaultImpl() {
        return _aidl_default_impl.r#onReceive(_arg_event);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
}
impl IUListener for BpUListener {
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onReceive(_arg_event)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onReceive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onReceive(_arg_event, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IUListenerAsync<P> for BpUListener {
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onReceive(_arg_event) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onReceive, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onReceive(_arg_event, _aidl_reply))
  }
}
impl IUListener for binder::binder_impl::Binder<BnUListener> {
  fn r#onReceive(&self, _arg_event: &parcelable_stubs::ParcelableUMessage) -> binder::Result<()> { self.0.r#onReceive(_arg_event) }
}
fn on_transact(_aidl_service: &dyn IUListener, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#onReceive => {
      let _arg_event: parcelable_stubs::ParcelableUMessage = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onReceive(&_arg_event);
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IUListener as _3_org_7_eclipse_9_uprotocol_4_core_4_ubus_10_IUListener;
}
