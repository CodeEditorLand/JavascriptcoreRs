// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/tauri-apps/gir-files)
// DO NOT EDIT

use glib::translate::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "JSCCheckSyntaxMode")]
pub enum CheckSyntaxMode {
	#[doc(alias = "JSC_CHECK_SYNTAX_MODE_SCRIPT")]
	Script,
	#[doc(alias = "JSC_CHECK_SYNTAX_MODE_MODULE")]
	Module,
	#[doc(hidden)]
	__Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for CheckSyntaxMode {
	type GlibType = ffi::JSCCheckSyntaxMode;

	#[inline]
	fn into_glib(self) -> ffi::JSCCheckSyntaxMode {
		match self {
			Self::Script => ffi::JSC_CHECK_SYNTAX_MODE_SCRIPT,
			Self::Module => ffi::JSC_CHECK_SYNTAX_MODE_MODULE,
			Self::__Unknown(value) => value,
		}
	}
}

#[doc(hidden)]
impl FromGlib<ffi::JSCCheckSyntaxMode> for CheckSyntaxMode {
	#[inline]
	unsafe fn from_glib(value:ffi::JSCCheckSyntaxMode) -> Self {
		match value {
			ffi::JSC_CHECK_SYNTAX_MODE_SCRIPT => Self::Script,
			ffi::JSC_CHECK_SYNTAX_MODE_MODULE => Self::Module,
			value => Self::__Unknown(value),
		}
	}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "JSCCheckSyntaxResult")]
pub enum CheckSyntaxResult {
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_SUCCESS")]
	Success,
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR")]
	RecoverableError,
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR")]
	IrrecoverableError,
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR")]
	UnterminatedLiteralError,
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR")]
	OutOfMemoryError,
	#[doc(alias = "JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR")]
	StackOverflowError,
	#[doc(hidden)]
	__Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for CheckSyntaxResult {
	type GlibType = ffi::JSCCheckSyntaxResult;

	#[inline]
	fn into_glib(self) -> ffi::JSCCheckSyntaxResult {
		match self {
			Self::Success => ffi::JSC_CHECK_SYNTAX_RESULT_SUCCESS,
			Self::RecoverableError => ffi::JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR,
			Self::IrrecoverableError => ffi::JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR,
			Self::UnterminatedLiteralError => {
				ffi::JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR
			},
			Self::OutOfMemoryError => ffi::JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR,
			Self::StackOverflowError => ffi::JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR,
			Self::__Unknown(value) => value,
		}
	}
}

#[doc(hidden)]
impl FromGlib<ffi::JSCCheckSyntaxResult> for CheckSyntaxResult {
	#[inline]
	unsafe fn from_glib(value:ffi::JSCCheckSyntaxResult) -> Self {
		match value {
			ffi::JSC_CHECK_SYNTAX_RESULT_SUCCESS => Self::Success,
			ffi::JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR => Self::RecoverableError,
			ffi::JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR => Self::IrrecoverableError,
			ffi::JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR => {
				Self::UnterminatedLiteralError
			},
			ffi::JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR => Self::OutOfMemoryError,
			ffi::JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR => Self::StackOverflowError,
			value => Self::__Unknown(value),
		}
	}
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "JSCOptionType")]
pub enum OptionType {
	#[doc(alias = "JSC_OPTION_BOOLEAN")]
	Boolean,
	#[doc(alias = "JSC_OPTION_INT")]
	Int,
	#[doc(alias = "JSC_OPTION_UINT")]
	Uint,
	#[doc(alias = "JSC_OPTION_SIZE")]
	Size,
	#[doc(alias = "JSC_OPTION_DOUBLE")]
	Double,
	#[doc(alias = "JSC_OPTION_STRING")]
	String,
	#[doc(alias = "JSC_OPTION_RANGE_STRING")]
	RangeString,
	#[doc(hidden)]
	__Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for OptionType {
	type GlibType = ffi::JSCOptionType;

	#[inline]
	fn into_glib(self) -> ffi::JSCOptionType {
		match self {
			Self::Boolean => ffi::JSC_OPTION_BOOLEAN,
			Self::Int => ffi::JSC_OPTION_INT,
			Self::Uint => ffi::JSC_OPTION_UINT,
			Self::Size => ffi::JSC_OPTION_SIZE,
			Self::Double => ffi::JSC_OPTION_DOUBLE,
			Self::String => ffi::JSC_OPTION_STRING,
			Self::RangeString => ffi::JSC_OPTION_RANGE_STRING,
			Self::__Unknown(value) => value,
		}
	}
}

#[doc(hidden)]
impl FromGlib<ffi::JSCOptionType> for OptionType {
	#[inline]
	unsafe fn from_glib(value:ffi::JSCOptionType) -> Self {
		match value {
			ffi::JSC_OPTION_BOOLEAN => Self::Boolean,
			ffi::JSC_OPTION_INT => Self::Int,
			ffi::JSC_OPTION_UINT => Self::Uint,
			ffi::JSC_OPTION_SIZE => Self::Size,
			ffi::JSC_OPTION_DOUBLE => Self::Double,
			ffi::JSC_OPTION_STRING => Self::String,
			ffi::JSC_OPTION_RANGE_STRING => Self::RangeString,
			value => Self::__Unknown(value),
		}
	}
}

#[cfg(feature = "v2_38")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "JSCTypedArrayType")]
pub enum TypedArrayType {
	#[doc(alias = "JSC_TYPED_ARRAY_NONE")]
	None,
	#[doc(alias = "JSC_TYPED_ARRAY_INT8")]
	Int8,
	#[doc(alias = "JSC_TYPED_ARRAY_INT16")]
	Int16,
	#[doc(alias = "JSC_TYPED_ARRAY_INT32")]
	Int32,
	#[doc(alias = "JSC_TYPED_ARRAY_INT64")]
	Int64,
	#[doc(alias = "JSC_TYPED_ARRAY_UINT8")]
	Uint8,
	#[doc(alias = "JSC_TYPED_ARRAY_UINT8_CLAMPED")]
	Uint8Clamped,
	#[doc(alias = "JSC_TYPED_ARRAY_UINT16")]
	Uint16,
	#[doc(alias = "JSC_TYPED_ARRAY_UINT32")]
	Uint32,
	#[doc(alias = "JSC_TYPED_ARRAY_UINT64")]
	Uint64,
	#[doc(alias = "JSC_TYPED_ARRAY_FLOAT32")]
	Float32,
	#[doc(alias = "JSC_TYPED_ARRAY_FLOAT64")]
	Float64,
	#[doc(hidden)]
	__Unknown(i32),
}

#[cfg(feature = "v2_38")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
#[doc(hidden)]
impl IntoGlib for TypedArrayType {
	type GlibType = ffi::JSCTypedArrayType;

	#[inline]
	fn into_glib(self) -> ffi::JSCTypedArrayType {
		match self {
			Self::None => ffi::JSC_TYPED_ARRAY_NONE,
			Self::Int8 => ffi::JSC_TYPED_ARRAY_INT8,
			Self::Int16 => ffi::JSC_TYPED_ARRAY_INT16,
			Self::Int32 => ffi::JSC_TYPED_ARRAY_INT32,
			Self::Int64 => ffi::JSC_TYPED_ARRAY_INT64,
			Self::Uint8 => ffi::JSC_TYPED_ARRAY_UINT8,
			Self::Uint8Clamped => ffi::JSC_TYPED_ARRAY_UINT8_CLAMPED,
			Self::Uint16 => ffi::JSC_TYPED_ARRAY_UINT16,
			Self::Uint32 => ffi::JSC_TYPED_ARRAY_UINT32,
			Self::Uint64 => ffi::JSC_TYPED_ARRAY_UINT64,
			Self::Float32 => ffi::JSC_TYPED_ARRAY_FLOAT32,
			Self::Float64 => ffi::JSC_TYPED_ARRAY_FLOAT64,
			Self::__Unknown(value) => value,
		}
	}
}

#[cfg(feature = "v2_38")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_38")))]
#[doc(hidden)]
impl FromGlib<ffi::JSCTypedArrayType> for TypedArrayType {
	#[inline]
	unsafe fn from_glib(value:ffi::JSCTypedArrayType) -> Self {
		match value {
			ffi::JSC_TYPED_ARRAY_NONE => Self::None,
			ffi::JSC_TYPED_ARRAY_INT8 => Self::Int8,
			ffi::JSC_TYPED_ARRAY_INT16 => Self::Int16,
			ffi::JSC_TYPED_ARRAY_INT32 => Self::Int32,
			ffi::JSC_TYPED_ARRAY_INT64 => Self::Int64,
			ffi::JSC_TYPED_ARRAY_UINT8 => Self::Uint8,
			ffi::JSC_TYPED_ARRAY_UINT8_CLAMPED => Self::Uint8Clamped,
			ffi::JSC_TYPED_ARRAY_UINT16 => Self::Uint16,
			ffi::JSC_TYPED_ARRAY_UINT32 => Self::Uint32,
			ffi::JSC_TYPED_ARRAY_UINT64 => Self::Uint64,
			ffi::JSC_TYPED_ARRAY_FLOAT32 => Self::Float32,
			ffi::JSC_TYPED_ARRAY_FLOAT64 => Self::Float64,
			value => Self::__Unknown(value),
		}
	}
}
