// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
	#[doc(alias = "JSCClass")]
	pub struct Class(Object<ffi::JSCClass, ffi::JSCClassClass>);

	match fn {
		type_ => || ffi::jsc_class_get_type(),
	}
}

impl Class {
	//#[doc(alias = "jsc_class_add_constructor")]
	// pub fn add_constructor<P: Fn() + 'static>(&self, name: Option<&str>,
	// callback: P, user_data: /*Unimplemented*/Option<Basic: Pointer>,
	// return_type: glib::types::Type, n_params: u32, : /*Unknown
	// conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Value> {
	//    unsafe { TODO: call ffi:jsc_class_add_constructor() }
	//}

	//#[doc(alias = "jsc_class_add_constructor_variadic")]
	// pub fn add_constructor_variadic<P: Fn() + 'static>(&self, name:
	// Option<&str>, callback: P, user_data: /*Unimplemented*/Option<Basic:
	// Pointer>, return_type: glib::types::Type) -> Option<Value> {    unsafe {
	// TODO: call ffi:jsc_class_add_constructor_variadic() }
	//}

	//#[doc(alias = "jsc_class_add_constructorv")]
	// pub fn add_constructorv<P: Fn() + 'static>(&self, name: Option<&str>,
	// callback: P, user_data: /*Unimplemented*/Option<Basic: Pointer>,
	// return_type: glib::types::Type, n_parameters: u32) -> Option<Value> {
	//    unsafe { TODO: call ffi:jsc_class_add_constructorv() }
	//}

	//#[doc(alias = "jsc_class_add_method")]
	// pub fn add_method<P: Fn() + 'static>(&self, name: &str, callback: P,
	// user_data: /*Unimplemented*/Option<Basic: Pointer>, return_type:
	// glib::types::Type, n_params: u32, : /*Unknown
	// conversion*//*Unimplemented*/Basic: VarArgs) {    unsafe { TODO: call
	// ffi:jsc_class_add_method() }
	//}

	//#[doc(alias = "jsc_class_add_method_variadic")]
	// pub fn add_method_variadic<P: Fn() + 'static>(&self, name: &str,
	// callback: P, user_data: /*Unimplemented*/Option<Basic: Pointer>,
	// return_type: glib::types::Type) {    unsafe { TODO: call
	// ffi:jsc_class_add_method_variadic() }
	//}

	//#[doc(alias = "jsc_class_add_methodv")]
	// pub fn add_methodv<P: Fn() + 'static>(&self, name: &str, callback: P,
	// user_data: /*Unimplemented*/Option<Basic: Pointer>, return_type:
	// glib::types::Type, n_parameters: u32) {    unsafe { TODO: call
	// ffi:jsc_class_add_methodv() }
	//}

	//#[doc(alias = "jsc_class_add_property")]
	// pub fn add_property(&self, name: &str, property_type: glib::types::Type,
	// getter: Option<Box_<dyn FnOnce() + 'static>>, setter: Option<Box_<dyn
	// Fn() + 'static>>, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
	//    unsafe { TODO: call ffi:jsc_class_add_property() }
	//}

	#[doc(alias = "jsc_class_get_name")]
	#[doc(alias = "get_name")]
	pub fn name(&self) -> Option<glib::GString> {
		unsafe {
			from_glib_none(ffi::jsc_class_get_name(self.to_glib_none().0))
		}
	}

	#[doc(alias = "jsc_class_get_parent")]
	#[doc(alias = "get_parent")]
	#[must_use]
	pub fn parent(&self) -> Option<Class> {
		unsafe {
			from_glib_none(ffi::jsc_class_get_parent(self.to_glib_none().0))
		}
	}
}
