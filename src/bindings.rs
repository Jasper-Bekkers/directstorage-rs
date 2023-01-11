#[inline]
pub unsafe fn DStorageCreateCompressionCodec(
    format: DSTORAGE_COMPRESSION_FORMAT,
    numthreads: u32,
    riid: *const ::windows::core::GUID,
    ppv: *mut *mut ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[link(name = "dstorage")]
    extern "system" {
        fn DStorageCreateCompressionCodec(
            format: DSTORAGE_COMPRESSION_FORMAT,
            numthreads: u32,
            riid: *const ::windows::core::GUID,
            ppv: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    DStorageCreateCompressionCodec(
        format,
        numthreads,
        ::core::mem::transmute(riid),
        ::core::mem::transmute(ppv),
    )
    .ok()
}
#[inline]
pub unsafe fn DStorageGetFactory(
    riid: *const ::windows::core::GUID,
    ppv: *mut *mut ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[link(name = "dstorage")]
    extern "system" {
        fn DStorageGetFactory(
            riid: *const ::windows::core::GUID,
            ppv: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT;
    }
    DStorageGetFactory(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
}
#[inline]
pub unsafe fn DStorageSetConfiguration(
    configuration: *const DSTORAGE_CONFIGURATION,
) -> ::windows::core::Result<()> {
    #[link(name = "dstorage")]
    extern "system" {
        fn DStorageSetConfiguration(
            configuration: *const DSTORAGE_CONFIGURATION,
        ) -> ::windows::core::HRESULT;
    }
    DStorageSetConfiguration(::core::mem::transmute(configuration)).ok()
}
#[repr(transparent)]
pub struct IDStorageCompressionCodec(::windows::core::IUnknown);
impl IDStorageCompressionCodec {
    pub unsafe fn CompressBuffer(
        &self,
        uncompresseddata: *const ::core::ffi::c_void,
        uncompresseddatasize: usize,
        compressionsetting: DSTORAGE_COMPRESSION,
        compressedbuffer: *mut ::core::ffi::c_void,
        compressedbuffersize: usize,
        compresseddatasize: *mut usize,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CompressBuffer)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(uncompresseddata),
            uncompresseddatasize,
            compressionsetting,
            ::core::mem::transmute(compressedbuffer),
            compressedbuffersize,
            ::core::mem::transmute(compresseddatasize),
        )
        .ok()
    }
    pub unsafe fn DecompressBuffer(
        &self,
        compresseddata: *const ::core::ffi::c_void,
        compresseddatasize: usize,
        uncompressedbuffer: *mut ::core::ffi::c_void,
        uncompressedbuffersize: usize,
        uncompresseddatasize: *mut usize,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DecompressBuffer)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(compresseddata),
            compresseddatasize,
            ::core::mem::transmute(uncompressedbuffer),
            uncompressedbuffersize,
            ::core::mem::transmute(uncompresseddatasize),
        )
        .ok()
    }
    pub unsafe fn CompressBufferBound(&self, uncompresseddatasize: usize) -> usize {
        (::windows::core::Vtable::vtable(self).CompressBufferBound)(
            ::windows::core::Vtable::as_raw(self),
            uncompresseddatasize,
        )
    }
}
::windows::core::interface_hierarchy!(IDStorageCompressionCodec, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageCompressionCodec {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageCompressionCodec {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageCompressionCodec {}
impl ::core::fmt::Debug for IDStorageCompressionCodec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageCompressionCodec")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageCompressionCodec {
    type Vtable = IDStorageCompressionCodec_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageCompressionCodec {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84ef5121_9b43_4d03_b5c1_cc34606b262d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCompressionCodec_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CompressBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uncompresseddata: *const ::core::ffi::c_void,
        uncompresseddatasize: usize,
        compressionsetting: DSTORAGE_COMPRESSION,
        compressedbuffer: *mut ::core::ffi::c_void,
        compressedbuffersize: usize,
        compresseddatasize: *mut usize,
    ) -> ::windows::core::HRESULT,
    pub DecompressBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compresseddata: *const ::core::ffi::c_void,
        compresseddatasize: usize,
        uncompressedbuffer: *mut ::core::ffi::c_void,
        uncompressedbuffersize: usize,
        uncompresseddatasize: *mut usize,
    ) -> ::windows::core::HRESULT,
    pub CompressBufferBound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uncompresseddatasize: usize,
    ) -> usize,
}
#[repr(transparent)]
pub struct IDStorageCustomDecompressionQueue(::windows::core::IUnknown);
impl IDStorageCustomDecompressionQueue {
    pub unsafe fn GetEvent(&self) -> ::windows::Win32::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).GetEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRequests(
        &self,
        requests: &mut [DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST],
        numrequests: *mut u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRequests)(
            ::windows::core::Vtable::as_raw(self),
            requests.len() as _,
            ::core::mem::transmute(requests.as_ptr()),
            ::core::mem::transmute(numrequests),
        )
        .ok()
    }
    pub unsafe fn SetRequestResults(
        &self,
        results: &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRequestResults)(
            ::windows::core::Vtable::as_raw(self),
            results.len() as _,
            ::core::mem::transmute(results.as_ptr()),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IDStorageCustomDecompressionQueue, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageCustomDecompressionQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageCustomDecompressionQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageCustomDecompressionQueue {}
impl ::core::fmt::Debug for IDStorageCustomDecompressionQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageCustomDecompressionQueue")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageCustomDecompressionQueue {
    type Vtable = IDStorageCustomDecompressionQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageCustomDecompressionQueue {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x97179b2f_2c21_49ca_8291_4e1bf4a160df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCustomDecompressionQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::HANDLE,
    pub GetRequests: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        maxrequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numrequests: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetRequestResults: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        numresults: u32,
        results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IDStorageCustomDecompressionQueue1(::windows::core::IUnknown);
impl IDStorageCustomDecompressionQueue1 {
    pub unsafe fn GetEvent(&self) -> ::windows::Win32::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).base__.GetEvent)(::windows::core::Vtable::as_raw(
            self,
        ))
    }
    pub unsafe fn GetRequests(
        &self,
        requests: &mut [DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST],
        numrequests: *mut u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRequests)(
            ::windows::core::Vtable::as_raw(self),
            requests.len() as _,
            ::core::mem::transmute(requests.as_ptr()),
            ::core::mem::transmute(numrequests),
        )
        .ok()
    }
    pub unsafe fn SetRequestResults(
        &self,
        results: &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT],
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self)
            .base__
            .SetRequestResults)(
            ::windows::core::Vtable::as_raw(self),
            results.len() as _,
            ::core::mem::transmute(results.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn GetRequests1(
        &self,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        requests: &mut [DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST],
        numrequests: *mut u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetRequests1)(
            ::windows::core::Vtable::as_raw(self),
            flags,
            requests.len() as _,
            ::core::mem::transmute(requests.as_ptr()),
            ::core::mem::transmute(numrequests),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(
    IDStorageCustomDecompressionQueue1,
    ::windows::core::IUnknown,
    IDStorageCustomDecompressionQueue
);
impl ::core::clone::Clone for IDStorageCustomDecompressionQueue1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageCustomDecompressionQueue1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageCustomDecompressionQueue1 {}
impl ::core::fmt::Debug for IDStorageCustomDecompressionQueue1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageCustomDecompressionQueue1")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageCustomDecompressionQueue1 {
    type Vtable = IDStorageCustomDecompressionQueue1_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageCustomDecompressionQueue1 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0d47c6c9_e61a_4706_93b4_68bfe3f4aa4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageCustomDecompressionQueue1_Vtbl {
    pub base__: IDStorageCustomDecompressionQueue_Vtbl,
    pub GetRequests1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxrequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numrequests: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IDStorageFactory(::windows::core::IUnknown);
impl IDStorageFactory {
    pub unsafe fn CreateQueue<T>(
        &self,
        desc: *const DSTORAGE_QUEUE_DESC,
    ) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).CreateQueue)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(desc),
            &<T as ::windows::core::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn OpenFile<'a, P0, T>(&self, path: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).OpenFile)(
            ::windows::core::Vtable::as_raw(self),
            path.into(),
            &<T as ::windows::core::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn CreateStatusArray<'a, P0, T>(
        &self,
        capacity: u32,
        name: P0,
    ) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::PCSTR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).CreateStatusArray)(
            ::windows::core::Vtable::as_raw(self),
            capacity,
            name.into(),
            &<T as ::windows::core::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    pub unsafe fn SetDebugFlags(&self, flags: u32) {
        (::windows::core::Vtable::vtable(self).SetDebugFlags)(
            ::windows::core::Vtable::as_raw(self),
            flags,
        )
    }
    pub unsafe fn SetStagingBufferSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStagingBufferSize)(
            ::windows::core::Vtable::as_raw(self),
            size,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IDStorageFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageFactory {}
impl ::core::fmt::Debug for IDStorageFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageFactory {
    type Vtable = IDStorageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6924ea0c_c3cd_4826_b10a_f64f4ed927c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        desc: *const ::core::mem::ManuallyDrop<DSTORAGE_QUEUE_DESC>,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OpenFile: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::windows::core::PCWSTR,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateStatusArray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        capacity: u32,
        name: ::windows::core::PCSTR,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetDebugFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32),
    pub SetStagingBufferSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: u32,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IDStorageFile(::windows::core::IUnknown);
impl IDStorageFile {
    pub unsafe fn Close(&self) {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFileInformation(
        &self,
    ) -> ::windows::core::Result<::windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION>
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileInformation)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<::windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION>(result__)
    }
}
::windows::core::interface_hierarchy!(IDStorageFile, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageFile {}
impl ::core::fmt::Debug for IDStorageFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageFile {
    type Vtable = IDStorageFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageFile {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5de95e7b_955a_4868_a73c_243b29f4b8da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageFile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetFileInformation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        info: *mut ::windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IDStorageQueue(::windows::core::IUnknown);
impl IDStorageQueue {
    pub unsafe fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST) {
        (::windows::core::Vtable::vtable(self).EnqueueRequest)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(request),
        )
    }
    pub unsafe fn EnqueueStatus<'a, P0>(&self, statusarray: P0, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDStorageStatusArray>>,
    {
        (::windows::core::Vtable::vtable(self).EnqueueStatus)(
            ::windows::core::Vtable::as_raw(self),
            statusarray.into().abi(),
            index,
        )
    }
    pub unsafe fn EnqueueSignal<'a, P0>(&self, fence: P0, value: u64)
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::Win32::Graphics::Direct3D12::ID3D12Fence>,
        >,
    {
        (::windows::core::Vtable::vtable(self).EnqueueSignal)(
            ::windows::core::Vtable::as_raw(self),
            fence.into().abi(),
            value,
        )
    }
    pub unsafe fn Submit(&self) {
        (::windows::core::Vtable::vtable(self).Submit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CancelRequestsWithTag(&self, mask: u64, value: u64) {
        (::windows::core::Vtable::vtable(self).CancelRequestsWithTag)(
            ::windows::core::Vtable::as_raw(self),
            mask,
            value,
        )
    }
    pub unsafe fn Close(&self) {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetErrorEvent(&self) -> ::windows::Win32::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).GetErrorEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn RetrieveErrorRecord(&self, record: *mut DSTORAGE_ERROR_RECORD) {
        (::windows::core::Vtable::vtable(self).RetrieveErrorRecord)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(record),
        )
    }
    pub unsafe fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO) {
        (::windows::core::Vtable::vtable(self).Query)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(info),
        )
    }
}
::windows::core::interface_hierarchy!(IDStorageQueue, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageQueue {}
impl ::core::fmt::Debug for IDStorageQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageQueue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageQueue {
    type Vtable = IDStorageQueue_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageQueue {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcfdbd83f_9e06_4fda_8ea5_69042137f49b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageQueue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnqueueRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: *const ::core::mem::ManuallyDrop<DSTORAGE_REQUEST>,
    ),
    pub EnqueueStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        statusarray: *mut ::core::ffi::c_void,
        index: u32,
    ),
    pub EnqueueSignal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fence: *mut ::core::ffi::c_void,
        value: u64,
    ),
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub CancelRequestsWithTag:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: u64, value: u64),
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetErrorEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::Win32::Foundation::HANDLE,
    pub RetrieveErrorRecord: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        record: *mut ::core::mem::ManuallyDrop<DSTORAGE_ERROR_RECORD>,
    ),
    pub Query: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        info: *mut ::core::mem::ManuallyDrop<DSTORAGE_QUEUE_INFO>,
    ),
}
#[repr(transparent)]
pub struct IDStorageQueue1(::windows::core::IUnknown);
impl IDStorageQueue1 {
    pub unsafe fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST) {
        (::windows::core::Vtable::vtable(self).base__.EnqueueRequest)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(request),
        )
    }
    pub unsafe fn EnqueueStatus<'a, P0>(&self, statusarray: P0, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDStorageStatusArray>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnqueueStatus)(
            ::windows::core::Vtable::as_raw(self),
            statusarray.into().abi(),
            index,
        )
    }
    pub unsafe fn EnqueueSignal<'a, P0>(&self, fence: P0, value: u64)
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::Win32::Graphics::Direct3D12::ID3D12Fence>,
        >,
    {
        (::windows::core::Vtable::vtable(self).base__.EnqueueSignal)(
            ::windows::core::Vtable::as_raw(self),
            fence.into().abi(),
            value,
        )
    }
    pub unsafe fn Submit(&self) {
        (::windows::core::Vtable::vtable(self).base__.Submit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CancelRequestsWithTag(&self, mask: u64, value: u64) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .CancelRequestsWithTag)(::windows::core::Vtable::as_raw(self), mask, value)
    }
    pub unsafe fn Close(&self) {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetErrorEvent(&self) -> ::windows::Win32::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).base__.GetErrorEvent)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn RetrieveErrorRecord(&self, record: *mut DSTORAGE_ERROR_RECORD) {
        (::windows::core::Vtable::vtable(self)
            .base__
            .RetrieveErrorRecord)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(record),
        )
    }
    pub unsafe fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO) {
        (::windows::core::Vtable::vtable(self).base__.Query)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(info),
        )
    }
    pub unsafe fn EnqueueSetEvent<'a, P0>(&self, handle: P0)
    where
        P0: ::std::convert::Into<::windows::Win32::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).EnqueueSetEvent)(
            ::windows::core::Vtable::as_raw(self),
            handle.into(),
        )
    }
}
::windows::core::interface_hierarchy!(IDStorageQueue1, ::windows::core::IUnknown, IDStorageQueue);
impl ::core::clone::Clone for IDStorageQueue1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageQueue1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageQueue1 {}
impl ::core::fmt::Debug for IDStorageQueue1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageQueue1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageQueue1 {
    type Vtable = IDStorageQueue1_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageQueue1 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd2f482c_5eff_41e8_9c9e_d2374b278128);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageQueue1_Vtbl {
    pub base__: IDStorageQueue_Vtbl,
    pub EnqueueSetEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: ::windows::Win32::Foundation::HANDLE,
    ),
}
#[repr(transparent)]
pub struct IDStorageStatusArray(::windows::core::IUnknown);
impl IDStorageStatusArray {
    pub unsafe fn IsComplete(&self, index: u32) -> bool {
        (::windows::core::Vtable::vtable(self).IsComplete)(
            ::windows::core::Vtable::as_raw(self),
            index,
        )
    }
    pub unsafe fn GetHResult(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetHResult)(
            ::windows::core::Vtable::as_raw(self),
            index,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IDStorageStatusArray, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDStorageStatusArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDStorageStatusArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDStorageStatusArray {}
impl ::core::fmt::Debug for IDStorageStatusArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDStorageStatusArray")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Vtable for IDStorageStatusArray {
    type Vtable = IDStorageStatusArray_Vtbl;
}
unsafe impl ::windows::core::Interface for IDStorageStatusArray {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x82397587_7cd5_453b_a02e_31379bd64656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDStorageStatusArray_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> bool,
    pub GetHResult: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
    ) -> ::windows::core::HRESULT,
}
pub const DSTORAGE_DISABLE_BUILTIN_CPU_DECOMPRESSION: i32 = -1i32;
pub const DSTORAGE_MAX_QUEUE_CAPACITY: u32 = 8192u32;
pub const DSTORAGE_MIN_QUEUE_CAPACITY: u32 = 128u32;
pub const DSTORAGE_REQUEST_MAX_NAME: u32 = 64u32;
pub const DSTORAGE_SDK_VERSION: u32 = 101u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_COMMAND_TYPE(pub i32);
pub const DSTORAGE_COMMAND_TYPE_NONE: DSTORAGE_COMMAND_TYPE = DSTORAGE_COMMAND_TYPE(-1i32);
pub const DSTORAGE_COMMAND_TYPE_REQUEST: DSTORAGE_COMMAND_TYPE = DSTORAGE_COMMAND_TYPE(0i32);
pub const DSTORAGE_COMMAND_TYPE_STATUS: DSTORAGE_COMMAND_TYPE = DSTORAGE_COMMAND_TYPE(1i32);
pub const DSTORAGE_COMMAND_TYPE_SIGNAL: DSTORAGE_COMMAND_TYPE = DSTORAGE_COMMAND_TYPE(2i32);
pub const DSTORAGE_COMMAND_TYPE_EVENT: DSTORAGE_COMMAND_TYPE = DSTORAGE_COMMAND_TYPE(3i32);
impl ::core::marker::Copy for DSTORAGE_COMMAND_TYPE {}
impl ::core::clone::Clone for DSTORAGE_COMMAND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_COMMAND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_COMMAND_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_COMMAND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_COMMAND_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_COMPRESSION(pub i32);
pub const DSTORAGE_COMPRESSION_FASTEST: DSTORAGE_COMPRESSION = DSTORAGE_COMPRESSION(-1i32);
pub const DSTORAGE_COMPRESSION_DEFAULT: DSTORAGE_COMPRESSION = DSTORAGE_COMPRESSION(0i32);
pub const DSTORAGE_COMPRESSION_BEST_RATIO: DSTORAGE_COMPRESSION = DSTORAGE_COMPRESSION(1i32);
impl ::core::marker::Copy for DSTORAGE_COMPRESSION {}
impl ::core::clone::Clone for DSTORAGE_COMPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_COMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_COMPRESSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_COMPRESSION")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_COMPRESSION_FORMAT(pub u8);
pub const DSTORAGE_COMPRESSION_FORMAT_NONE: DSTORAGE_COMPRESSION_FORMAT =
    DSTORAGE_COMPRESSION_FORMAT(0u8);
pub const DSTORAGE_COMPRESSION_FORMAT_GDEFLATE: DSTORAGE_COMPRESSION_FORMAT =
    DSTORAGE_COMPRESSION_FORMAT(1u8);
pub const DSTORAGE_CUSTOM_COMPRESSION_0: DSTORAGE_COMPRESSION_FORMAT =
    DSTORAGE_COMPRESSION_FORMAT(128u8);
impl ::core::marker::Copy for DSTORAGE_COMPRESSION_FORMAT {}
impl ::core::clone::Clone for DSTORAGE_COMPRESSION_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_COMPRESSION_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_COMPRESSION_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_COMPRESSION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_COMPRESSION_FORMAT")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS(pub u32);
pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_NONE: DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS =
    DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS(0u32);
pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_DEST_IN_UPLOAD_HEAP:
    DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS = DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS(1u32);
impl ::core::marker::Copy for DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {}
impl ::core::clone::Clone for DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_DEBUG(pub i32);
pub const DSTORAGE_DEBUG_NONE: DSTORAGE_DEBUG = DSTORAGE_DEBUG(0i32);
pub const DSTORAGE_DEBUG_SHOW_ERRORS: DSTORAGE_DEBUG = DSTORAGE_DEBUG(1i32);
pub const DSTORAGE_DEBUG_BREAK_ON_ERROR: DSTORAGE_DEBUG = DSTORAGE_DEBUG(2i32);
pub const DSTORAGE_DEBUG_RECORD_OBJECT_NAMES: DSTORAGE_DEBUG = DSTORAGE_DEBUG(4i32);
impl ::core::marker::Copy for DSTORAGE_DEBUG {}
impl ::core::clone::Clone for DSTORAGE_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_DEBUG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DEBUG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_DEBUG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_DEBUG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_GET_REQUEST_FLAGS(pub u32);
pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM: DSTORAGE_GET_REQUEST_FLAGS =
    DSTORAGE_GET_REQUEST_FLAGS(1u32);
pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN: DSTORAGE_GET_REQUEST_FLAGS =
    DSTORAGE_GET_REQUEST_FLAGS(2u32);
pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL: DSTORAGE_GET_REQUEST_FLAGS =
    DSTORAGE_GET_REQUEST_FLAGS(3u32);
impl ::core::marker::Copy for DSTORAGE_GET_REQUEST_FLAGS {}
impl ::core::clone::Clone for DSTORAGE_GET_REQUEST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_GET_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_GET_REQUEST_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_GET_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_GET_REQUEST_FLAGS")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_PRIORITY(pub i8);
pub const DSTORAGE_PRIORITY_LOW: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(-1i8);
pub const DSTORAGE_PRIORITY_NORMAL: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(0i8);
pub const DSTORAGE_PRIORITY_HIGH: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(1i8);
pub const DSTORAGE_PRIORITY_REALTIME: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(2i8);
pub const DSTORAGE_PRIORITY_FIRST: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(-1i8);
pub const DSTORAGE_PRIORITY_LAST: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(2i8);
pub const DSTORAGE_PRIORITY_COUNT: DSTORAGE_PRIORITY = DSTORAGE_PRIORITY(4i8);
impl ::core::marker::Copy for DSTORAGE_PRIORITY {}
impl ::core::clone::Clone for DSTORAGE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_REQUEST_DESTINATION_TYPE(pub u64);
pub const DSTORAGE_REQUEST_DESTINATION_MEMORY: DSTORAGE_REQUEST_DESTINATION_TYPE =
    DSTORAGE_REQUEST_DESTINATION_TYPE(0u64);
pub const DSTORAGE_REQUEST_DESTINATION_BUFFER: DSTORAGE_REQUEST_DESTINATION_TYPE =
    DSTORAGE_REQUEST_DESTINATION_TYPE(1u64);
pub const DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION: DSTORAGE_REQUEST_DESTINATION_TYPE =
    DSTORAGE_REQUEST_DESTINATION_TYPE(2u64);
pub const DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES: DSTORAGE_REQUEST_DESTINATION_TYPE =
    DSTORAGE_REQUEST_DESTINATION_TYPE(3u64);
pub const DSTORAGE_REQUEST_DESTINATION_TILES: DSTORAGE_REQUEST_DESTINATION_TYPE =
    DSTORAGE_REQUEST_DESTINATION_TYPE(4u64);
impl ::core::marker::Copy for DSTORAGE_REQUEST_DESTINATION_TYPE {}
impl ::core::clone::Clone for DSTORAGE_REQUEST_DESTINATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_REQUEST_DESTINATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_REQUEST_DESTINATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_REQUEST_DESTINATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_REQUEST_DESTINATION_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_REQUEST_SOURCE_TYPE(pub u64);
pub const DSTORAGE_REQUEST_SOURCE_FILE: DSTORAGE_REQUEST_SOURCE_TYPE =
    DSTORAGE_REQUEST_SOURCE_TYPE(0u64);
pub const DSTORAGE_REQUEST_SOURCE_MEMORY: DSTORAGE_REQUEST_SOURCE_TYPE =
    DSTORAGE_REQUEST_SOURCE_TYPE(1u64);
impl ::core::marker::Copy for DSTORAGE_REQUEST_SOURCE_TYPE {}
impl ::core::clone::Clone for DSTORAGE_REQUEST_SOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_REQUEST_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_REQUEST_SOURCE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_REQUEST_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_REQUEST_SOURCE_TYPE")
            .field(&self.0)
            .finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSTORAGE_STAGING_BUFFER_SIZE(pub u32);
pub const DSTORAGE_STAGING_BUFFER_SIZE_0: DSTORAGE_STAGING_BUFFER_SIZE =
    DSTORAGE_STAGING_BUFFER_SIZE(0u32);
pub const DSTORAGE_STAGING_BUFFER_SIZE_32MB: DSTORAGE_STAGING_BUFFER_SIZE =
    DSTORAGE_STAGING_BUFFER_SIZE(33554432u32);
impl ::core::marker::Copy for DSTORAGE_STAGING_BUFFER_SIZE {}
impl ::core::clone::Clone for DSTORAGE_STAGING_BUFFER_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSTORAGE_STAGING_BUFFER_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_STAGING_BUFFER_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSTORAGE_STAGING_BUFFER_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSTORAGE_STAGING_BUFFER_SIZE")
            .field(&self.0)
            .finish()
    }
}
#[repr(C)]
pub struct DSTORAGE_CONFIGURATION {
    pub NumSubmitThreads: u32,
    pub NumBuiltInCpuDecompressionThreads: i32,
    pub ForceMappingLayer: ::windows::Win32::Foundation::BOOL,
    pub DisableBypassIO: ::windows::Win32::Foundation::BOOL,
    pub DisableTelemetry: ::windows::Win32::Foundation::BOOL,
    pub DisableGpuDecompressionMetacommand: ::windows::Win32::Foundation::BOOL,
    pub DisableGpuDecompression: ::windows::Win32::Foundation::BOOL,
}
impl ::core::marker::Copy for DSTORAGE_CONFIGURATION {}
impl ::core::clone::Clone for DSTORAGE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_CONFIGURATION")
            .field("NumSubmitThreads", &self.NumSubmitThreads)
            .field(
                "NumBuiltInCpuDecompressionThreads",
                &self.NumBuiltInCpuDecompressionThreads,
            )
            .field("ForceMappingLayer", &self.ForceMappingLayer)
            .field("DisableBypassIO", &self.DisableBypassIO)
            .field("DisableTelemetry", &self.DisableTelemetry)
            .field(
                "DisableGpuDecompressionMetacommand",
                &self.DisableGpuDecompressionMetacommand,
            )
            .field("DisableGpuDecompression", &self.DisableGpuDecompression)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_CONFIGURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_CONFIGURATION>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_CONFIGURATION {}
impl ::core::default::Default for DSTORAGE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    pub Id: u64,
    pub CompressionFormat: DSTORAGE_COMPRESSION_FORMAT,
    pub Reserved: [u8; 3],
    pub Flags: DSTORAGE_CUSTOM_DECOMPRESSION_FLAGS,
    pub SrcSize: u64,
    pub SrcBuffer: *const ::core::ffi::c_void,
    pub DstSize: u64,
    pub DstBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {}
impl ::core::clone::Clone for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST")
            .field("Id", &self.Id)
            .field("CompressionFormat", &self.CompressionFormat)
            .field("Reserved", &self.Reserved)
            .field("Flags", &self.Flags)
            .field("SrcSize", &self.SrcSize)
            .field("SrcBuffer", &self.SrcBuffer)
            .field("DstSize", &self.DstSize)
            .field("DstBuffer", &self.DstBuffer)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {}
impl ::core::default::Default for DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    pub Id: u64,
    pub Result: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {}
impl ::core::clone::Clone for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_CUSTOM_DECOMPRESSION_RESULT")
            .field("Id", &self.Id)
            .field("Result", &self.Result)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {}
impl ::core::default::Default for DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DSTORAGE_DESTINATION {
    pub Memory: DSTORAGE_DESTINATION_MEMORY,
    pub Buffer: ::core::mem::ManuallyDrop<DSTORAGE_DESTINATION_BUFFER>,
    pub Texture: ::core::mem::ManuallyDrop<DSTORAGE_DESTINATION_TEXTURE_REGION>,
    pub MultipleSubresources: ::core::mem::ManuallyDrop<DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES>,
    pub Tiles: ::core::mem::ManuallyDrop<DSTORAGE_DESTINATION_TILES>,
}
impl ::core::clone::Clone for DSTORAGE_DESTINATION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_DESTINATION>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION {}
impl ::core::default::Default for DSTORAGE_DESTINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_DESTINATION_BUFFER {
    pub Resource: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Resource>,
    pub Offset: u64,
    pub Size: u32,
}
impl ::core::clone::Clone for DSTORAGE_DESTINATION_BUFFER {
    fn clone(&self) -> Self {
        Self {
            Resource: self.Resource.clone(),
            Offset: self.Offset,
            Size: self.Size,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_DESTINATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_DESTINATION_BUFFER")
            .field("Resource", &self.Resource)
            .field("Offset", &self.Offset)
            .field("Size", &self.Size)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION_BUFFER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Resource == other.Resource && self.Offset == other.Offset && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION_BUFFER {}
impl ::core::default::Default for DSTORAGE_DESTINATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_DESTINATION_MEMORY {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Size: u32,
}
impl ::core::marker::Copy for DSTORAGE_DESTINATION_MEMORY {}
impl ::core::clone::Clone for DSTORAGE_DESTINATION_MEMORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_DESTINATION_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_DESTINATION_MEMORY")
            .field("Buffer", &self.Buffer)
            .field("Size", &self.Size)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION_MEMORY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_DESTINATION_MEMORY>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION_MEMORY {}
impl ::core::default::Default for DSTORAGE_DESTINATION_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    pub Resource: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Resource>,
    pub FirstSubresource: u32,
}
impl ::core::clone::Clone for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    fn clone(&self) -> Self {
        Self {
            Resource: self.Resource.clone(),
            FirstSubresource: self.FirstSubresource,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES")
            .field("Resource", &self.Resource)
            .field("FirstSubresource", &self.FirstSubresource)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.Resource == other.Resource && self.FirstSubresource == other.FirstSubresource
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {}
impl ::core::default::Default for DSTORAGE_DESTINATION_MULTIPLE_SUBRESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_DESTINATION_TEXTURE_REGION {
    pub Resource: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Resource>,
    pub SubresourceIndex: u32,
    pub Region: ::windows::Win32::Graphics::Direct3D12::D3D12_BOX,
}
impl ::core::clone::Clone for DSTORAGE_DESTINATION_TEXTURE_REGION {
    fn clone(&self) -> Self {
        Self {
            Resource: self.Resource.clone(),
            SubresourceIndex: self.SubresourceIndex,
            Region: self.Region,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_DESTINATION_TEXTURE_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_DESTINATION_TEXTURE_REGION")
            .field("Resource", &self.Resource)
            .field("SubresourceIndex", &self.SubresourceIndex)
            .field("Region", &self.Region)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION_TEXTURE_REGION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION_TEXTURE_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Resource == other.Resource
            && self.SubresourceIndex == other.SubresourceIndex
            && self.Region == other.Region
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION_TEXTURE_REGION {}
impl ::core::default::Default for DSTORAGE_DESTINATION_TEXTURE_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_DESTINATION_TILES {
    pub Resource: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Resource>,
    pub TiledRegionStartCoordinate:
        ::windows::Win32::Graphics::Direct3D12::D3D12_TILED_RESOURCE_COORDINATE,
    pub TileRegionSize: ::windows::Win32::Graphics::Direct3D12::D3D12_TILE_REGION_SIZE,
}
impl ::core::clone::Clone for DSTORAGE_DESTINATION_TILES {
    fn clone(&self) -> Self {
        Self {
            Resource: self.Resource.clone(),
            TiledRegionStartCoordinate: self.TiledRegionStartCoordinate,
            TileRegionSize: self.TileRegionSize,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_DESTINATION_TILES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_DESTINATION_TILES")
            .field("Resource", &self.Resource)
            .field(
                "TiledRegionStartCoordinate",
                &self.TiledRegionStartCoordinate,
            )
            .field("TileRegionSize", &self.TileRegionSize)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_DESTINATION_TILES {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_DESTINATION_TILES {
    fn eq(&self, other: &Self) -> bool {
        self.Resource == other.Resource
            && self.TiledRegionStartCoordinate == other.TiledRegionStartCoordinate
            && self.TileRegionSize == other.TileRegionSize
    }
}
impl ::core::cmp::Eq for DSTORAGE_DESTINATION_TILES {}
impl ::core::default::Default for DSTORAGE_DESTINATION_TILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_FIRST_FAILURE {
    pub HResult: ::windows::core::HRESULT,
    pub CommandType: DSTORAGE_COMMAND_TYPE,
    pub Anonymous: DSTORAGE_ERROR_FIRST_FAILURE_0,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_FIRST_FAILURE {
    fn clone(&self) -> Self {
        Self {
            HResult: self.HResult,
            CommandType: self.CommandType,
            Anonymous: self.Anonymous.clone(),
        }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_FIRST_FAILURE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_FIRST_FAILURE {
    fn eq(&self, other: &Self) -> bool {
        self.HResult == other.HResult
            && self.CommandType == other.CommandType
            && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_FIRST_FAILURE {}
impl ::core::default::Default for DSTORAGE_ERROR_FIRST_FAILURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DSTORAGE_ERROR_FIRST_FAILURE_0 {
    pub Request: ::core::mem::ManuallyDrop<DSTORAGE_ERROR_PARAMETERS_REQUEST>,
    pub Status: ::core::mem::ManuallyDrop<DSTORAGE_ERROR_PARAMETERS_STATUS>,
    pub Signal: ::core::mem::ManuallyDrop<DSTORAGE_ERROR_PARAMETERS_SIGNAL>,
    pub Event: DSTORAGE_ERROR_PARAMETERS_EVENT,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_FIRST_FAILURE_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_FIRST_FAILURE_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_FIRST_FAILURE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_ERROR_FIRST_FAILURE_0>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_FIRST_FAILURE_0 {}
impl ::core::default::Default for DSTORAGE_ERROR_FIRST_FAILURE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_EVENT {
    pub Handle: ::windows::Win32::Foundation::HANDLE,
}
impl ::core::marker::Copy for DSTORAGE_ERROR_PARAMETERS_EVENT {}
impl ::core::clone::Clone for DSTORAGE_ERROR_PARAMETERS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_ERROR_PARAMETERS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_ERROR_PARAMETERS_EVENT")
            .field("Handle", &self.Handle)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_PARAMETERS_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_PARAMETERS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_PARAMETERS_EVENT {}
impl ::core::default::Default for DSTORAGE_ERROR_PARAMETERS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_REQUEST {
    pub Filename: [u16; 260],
    pub RequestName: [::windows::Win32::Foundation::CHAR; 64],
    pub Request: DSTORAGE_REQUEST,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_PARAMETERS_REQUEST {
    fn clone(&self) -> Self {
        Self {
            Filename: self.Filename,
            RequestName: self.RequestName,
            Request: self.Request.clone(),
        }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_PARAMETERS_REQUEST {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_PARAMETERS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Filename == other.Filename
            && self.RequestName == other.RequestName
            && self.Request == other.Request
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_PARAMETERS_REQUEST {}
impl ::core::default::Default for DSTORAGE_ERROR_PARAMETERS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    pub Fence: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Fence>,
    pub Value: u64,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    fn clone(&self) -> Self {
        Self {
            Fence: self.Fence.clone(),
            Value: self.Value,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_ERROR_PARAMETERS_SIGNAL")
            .field("Fence", &self.Fence)
            .field("Value", &self.Value)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    fn eq(&self, other: &Self) -> bool {
        self.Fence == other.Fence && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_PARAMETERS_SIGNAL {}
impl ::core::default::Default for DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_PARAMETERS_STATUS {
    pub StatusArray: ::core::option::Option<IDStorageStatusArray>,
    pub Index: u32,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_PARAMETERS_STATUS {
    fn clone(&self) -> Self {
        Self {
            StatusArray: self.StatusArray.clone(),
            Index: self.Index,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_ERROR_PARAMETERS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_ERROR_PARAMETERS_STATUS")
            .field("StatusArray", &self.StatusArray)
            .field("Index", &self.Index)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_PARAMETERS_STATUS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_PARAMETERS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.StatusArray == other.StatusArray && self.Index == other.Index
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_PARAMETERS_STATUS {}
impl ::core::default::Default for DSTORAGE_ERROR_PARAMETERS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_ERROR_RECORD {
    pub FailureCount: u32,
    pub FirstFailure: DSTORAGE_ERROR_FIRST_FAILURE,
}
impl ::core::clone::Clone for DSTORAGE_ERROR_RECORD {
    fn clone(&self) -> Self {
        Self {
            FailureCount: self.FailureCount,
            FirstFailure: self.FirstFailure.clone(),
        }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_ERROR_RECORD {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_ERROR_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.FailureCount == other.FailureCount && self.FirstFailure == other.FirstFailure
    }
}
impl ::core::cmp::Eq for DSTORAGE_ERROR_RECORD {}
impl ::core::default::Default for DSTORAGE_ERROR_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_QUEUE_DESC {
    pub SourceType: DSTORAGE_REQUEST_SOURCE_TYPE,
    pub Capacity: u16,
    pub Priority: DSTORAGE_PRIORITY,
    pub Name: ::windows::core::PCSTR,
    pub Device: ::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Device>,
}
impl ::core::clone::Clone for DSTORAGE_QUEUE_DESC {
    fn clone(&self) -> Self {
        Self {
            SourceType: self.SourceType,
            Capacity: self.Capacity,
            Priority: self.Priority,
            Name: self.Name,
            Device: self.Device.clone(),
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_QUEUE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_QUEUE_DESC")
            .field("SourceType", &self.SourceType)
            .field("Capacity", &self.Capacity)
            .field("Priority", &self.Priority)
            .field("Name", &self.Name)
            .field("Device", &self.Device)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_QUEUE_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_QUEUE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SourceType == other.SourceType
            && self.Capacity == other.Capacity
            && self.Priority == other.Priority
            && self.Name == other.Name
            && self.Device == other.Device
    }
}
impl ::core::cmp::Eq for DSTORAGE_QUEUE_DESC {}
impl ::core::default::Default for DSTORAGE_QUEUE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_QUEUE_INFO {
    pub Desc: DSTORAGE_QUEUE_DESC,
    pub EmptySlotCount: u16,
    pub RequestCountUntilAutoSubmit: u16,
}
impl ::core::clone::Clone for DSTORAGE_QUEUE_INFO {
    fn clone(&self) -> Self {
        Self {
            Desc: self.Desc.clone(),
            EmptySlotCount: self.EmptySlotCount,
            RequestCountUntilAutoSubmit: self.RequestCountUntilAutoSubmit,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_QUEUE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_QUEUE_INFO")
            .field("Desc", &self.Desc)
            .field("EmptySlotCount", &self.EmptySlotCount)
            .field(
                "RequestCountUntilAutoSubmit",
                &self.RequestCountUntilAutoSubmit,
            )
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_QUEUE_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_QUEUE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Desc == other.Desc
            && self.EmptySlotCount == other.EmptySlotCount
            && self.RequestCountUntilAutoSubmit == other.RequestCountUntilAutoSubmit
    }
}
impl ::core::cmp::Eq for DSTORAGE_QUEUE_INFO {}
impl ::core::default::Default for DSTORAGE_QUEUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_REQUEST {
    pub Options: DSTORAGE_REQUEST_OPTIONS,
    pub Source: DSTORAGE_SOURCE,
    pub Destination: DSTORAGE_DESTINATION,
    pub UncompressedSize: u32,
    pub CancellationTag: u64,
    pub Name: ::windows::core::PCSTR,
}
impl ::core::clone::Clone for DSTORAGE_REQUEST {
    fn clone(&self) -> Self {
        Self {
            Options: self.Options,
            Source: self.Source.clone(),
            Destination: self.Destination.clone(),
            UncompressedSize: self.UncompressedSize,
            CancellationTag: self.CancellationTag,
            Name: self.Name,
        }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_REQUEST {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Options == other.Options
            && self.Source == other.Source
            && self.Destination == other.Destination
            && self.UncompressedSize == other.UncompressedSize
            && self.CancellationTag == other.CancellationTag
            && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DSTORAGE_REQUEST {}
impl ::core::default::Default for DSTORAGE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_REQUEST_OPTIONS {
    pub Upper: u64,
    pub Lower: u64,
}
impl ::core::marker::Copy for DSTORAGE_REQUEST_OPTIONS {}
impl ::core::clone::Clone for DSTORAGE_REQUEST_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_REQUEST_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_REQUEST_OPTIONS")
            .field("Upper", &self.Upper)
            .field("Lower", &self.Lower)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_REQUEST_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_REQUEST_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_REQUEST_OPTIONS>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_REQUEST_OPTIONS {}
impl ::core::default::Default for DSTORAGE_REQUEST_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DSTORAGE_SOURCE {
    pub Memory: DSTORAGE_SOURCE_MEMORY,
    pub File: ::core::mem::ManuallyDrop<DSTORAGE_SOURCE_FILE>,
}
impl ::core::clone::Clone for DSTORAGE_SOURCE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_SOURCE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_SOURCE>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_SOURCE {}
impl ::core::default::Default for DSTORAGE_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_SOURCE_FILE {
    pub Source: ::core::option::Option<IDStorageFile>,
    pub Offset: u64,
    pub Size: u32,
}
impl ::core::clone::Clone for DSTORAGE_SOURCE_FILE {
    fn clone(&self) -> Self {
        Self {
            Source: self.Source.clone(),
            Offset: self.Offset,
            Size: self.Size,
        }
    }
}
impl ::core::fmt::Debug for DSTORAGE_SOURCE_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_SOURCE_FILE")
            .field("Source", &self.Source)
            .field("Offset", &self.Offset)
            .field("Size", &self.Size)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_SOURCE_FILE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DSTORAGE_SOURCE_FILE {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source && self.Offset == other.Offset && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for DSTORAGE_SOURCE_FILE {}
impl ::core::default::Default for DSTORAGE_SOURCE_FILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSTORAGE_SOURCE_MEMORY {
    pub Source: *const ::core::ffi::c_void,
    pub Size: u32,
}
impl ::core::marker::Copy for DSTORAGE_SOURCE_MEMORY {}
impl ::core::clone::Clone for DSTORAGE_SOURCE_MEMORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSTORAGE_SOURCE_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSTORAGE_SOURCE_MEMORY")
            .field("Source", &self.Source)
            .field("Size", &self.Size)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSTORAGE_SOURCE_MEMORY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSTORAGE_SOURCE_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<DSTORAGE_SOURCE_MEMORY>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for DSTORAGE_SOURCE_MEMORY {}
impl ::core::default::Default for DSTORAGE_SOURCE_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub trait IDStorageCompressionCodec_Impl: Sized {
    fn CompressBuffer(
        &self,
        uncompresseddata: *const ::core::ffi::c_void,
        uncompresseddatasize: usize,
        compressionsetting: DSTORAGE_COMPRESSION,
        compressedbuffer: *mut ::core::ffi::c_void,
        compressedbuffersize: usize,
        compresseddatasize: *mut usize,
    ) -> ::windows::core::Result<()>;
    fn DecompressBuffer(
        &self,
        compresseddata: *const ::core::ffi::c_void,
        compresseddatasize: usize,
        uncompressedbuffer: *mut ::core::ffi::c_void,
        uncompressedbuffersize: usize,
        uncompresseddatasize: *mut usize,
    ) -> ::windows::core::Result<()>;
    fn CompressBufferBound(&self, uncompresseddatasize: usize) -> usize;
}
impl ::windows::core::RuntimeName for IDStorageCompressionCodec {}
impl IDStorageCompressionCodec_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCompressionCodec_Impl,
        const OFFSET: isize,
    >() -> IDStorageCompressionCodec_Vtbl {
        unsafe extern "system" fn CompressBuffer<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            uncompresseddata: *const ::core::ffi::c_void,
            uncompresseddatasize: usize,
            compressionsetting: DSTORAGE_COMPRESSION,
            compressedbuffer: *mut ::core::ffi::c_void,
            compressedbuffersize: usize,
            compresseddatasize: *mut usize,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompressBuffer(
                ::core::mem::transmute_copy(&uncompresseddata),
                ::core::mem::transmute_copy(&uncompresseddatasize),
                ::core::mem::transmute_copy(&compressionsetting),
                ::core::mem::transmute_copy(&compressedbuffer),
                ::core::mem::transmute_copy(&compressedbuffersize),
                ::core::mem::transmute_copy(&compresseddatasize),
            )
            .into()
        }
        unsafe extern "system" fn DecompressBuffer<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            compresseddata: *const ::core::ffi::c_void,
            compresseddatasize: usize,
            uncompressedbuffer: *mut ::core::ffi::c_void,
            uncompressedbuffersize: usize,
            uncompresseddatasize: *mut usize,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecompressBuffer(
                ::core::mem::transmute_copy(&compresseddata),
                ::core::mem::transmute_copy(&compresseddatasize),
                ::core::mem::transmute_copy(&uncompressedbuffer),
                ::core::mem::transmute_copy(&uncompressedbuffersize),
                ::core::mem::transmute_copy(&uncompresseddatasize),
            )
            .into()
        }
        unsafe extern "system" fn CompressBufferBound<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCompressionCodec_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            uncompresseddatasize: usize,
        ) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompressBufferBound(::core::mem::transmute_copy(&uncompresseddatasize))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompressBuffer: CompressBuffer::<Identity, Impl, OFFSET>,
            DecompressBuffer: DecompressBuffer::<Identity, Impl, OFFSET>,
            CompressBufferBound: CompressBufferBound::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageCompressionCodec as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageCustomDecompressionQueue_Impl: Sized {
    fn GetEvent(&self) -> ::windows::Win32::Foundation::HANDLE;
    fn GetRequests(
        &self,
        maxrequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numrequests: *mut u32,
    ) -> ::windows::core::Result<()>;
    fn SetRequestResults(
        &self,
        numresults: u32,
        results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDStorageCustomDecompressionQueue {}
impl IDStorageCustomDecompressionQueue_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCustomDecompressionQueue_Impl,
        const OFFSET: isize,
    >() -> IDStorageCustomDecompressionQueue_Vtbl {
        unsafe extern "system" fn GetEvent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::Win32::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEvent()
        }
        unsafe extern "system" fn GetRequests<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            maxrequests: u32,
            requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
            numrequests: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequests(
                ::core::mem::transmute_copy(&maxrequests),
                ::core::mem::transmute_copy(&requests),
                ::core::mem::transmute_copy(&numrequests),
            )
            .into()
        }
        unsafe extern "system" fn SetRequestResults<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            numresults: u32,
            results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestResults(
                ::core::mem::transmute_copy(&numresults),
                ::core::mem::transmute_copy(&results),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetRequests: GetRequests::<Identity, Impl, OFFSET>,
            SetRequestResults: SetRequestResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageCustomDecompressionQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageCustomDecompressionQueue1_Impl:
    Sized + IDStorageCustomDecompressionQueue_Impl
{
    fn GetRequests1(
        &self,
        flags: DSTORAGE_GET_REQUEST_FLAGS,
        maxrequests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        numrequests: *mut u32,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDStorageCustomDecompressionQueue1 {}
impl IDStorageCustomDecompressionQueue1_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageCustomDecompressionQueue1_Impl,
        const OFFSET: isize,
    >() -> IDStorageCustomDecompressionQueue1_Vtbl {
        unsafe extern "system" fn GetRequests1<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageCustomDecompressionQueue1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            flags: DSTORAGE_GET_REQUEST_FLAGS,
            maxrequests: u32,
            requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
            numrequests: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequests1(
                ::core::mem::transmute_copy(&flags),
                ::core::mem::transmute_copy(&maxrequests),
                ::core::mem::transmute_copy(&requests),
                ::core::mem::transmute_copy(&numrequests),
            )
            .into()
        }
        Self {
            base__: IDStorageCustomDecompressionQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRequests1: GetRequests1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageCustomDecompressionQueue1 as ::windows::core::Interface>::IID
            || iid == &<IDStorageCustomDecompressionQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageFactory_Impl: Sized {
    fn CreateQueue(
        &self,
        desc: *const DSTORAGE_QUEUE_DESC,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
    fn OpenFile(
        &self,
        path: &::windows::core::PCWSTR,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
    fn CreateStatusArray(
        &self,
        capacity: u32,
        name: &::windows::core::PCSTR,
        riid: *const ::windows::core::GUID,
        ppv: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::Result<()>;
    fn SetDebugFlags(&self, flags: u32);
    fn SetStagingBufferSize(&self, size: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDStorageFactory {}
impl IDStorageFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageFactory_Impl,
        const OFFSET: isize,
    >() -> IDStorageFactory_Vtbl {
        unsafe extern "system" fn CreateQueue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            desc: *const ::core::mem::ManuallyDrop<DSTORAGE_QUEUE_DESC>,
            riid: *const ::windows::core::GUID,
            ppv: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateQueue(
                ::core::mem::transmute_copy(&desc),
                ::core::mem::transmute_copy(&riid),
                ::core::mem::transmute_copy(&ppv),
            )
            .into()
        }
        unsafe extern "system" fn OpenFile<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            path: ::windows::core::PCWSTR,
            riid: *const ::windows::core::GUID,
            ppv: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenFile(
                ::core::mem::transmute(&path),
                ::core::mem::transmute_copy(&riid),
                ::core::mem::transmute_copy(&ppv),
            )
            .into()
        }
        unsafe extern "system" fn CreateStatusArray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            capacity: u32,
            name: ::windows::core::PCSTR,
            riid: *const ::windows::core::GUID,
            ppv: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStatusArray(
                ::core::mem::transmute_copy(&capacity),
                ::core::mem::transmute(&name),
                ::core::mem::transmute_copy(&riid),
                ::core::mem::transmute_copy(&ppv),
            )
            .into()
        }
        unsafe extern "system" fn SetDebugFlags<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            flags: u32,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugFlags(::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetStagingBufferSize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            size: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStagingBufferSize(::core::mem::transmute_copy(&size))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateQueue: CreateQueue::<Identity, Impl, OFFSET>,
            OpenFile: OpenFile::<Identity, Impl, OFFSET>,
            CreateStatusArray: CreateStatusArray::<Identity, Impl, OFFSET>,
            SetDebugFlags: SetDebugFlags::<Identity, Impl, OFFSET>,
            SetStagingBufferSize: SetStagingBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageFile_Impl: Sized {
    fn Close(&self);
    fn GetFileInformation(
        &self,
    ) -> ::windows::core::Result<::windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION>;
}
impl ::windows::core::RuntimeName for IDStorageFile {}
impl IDStorageFile_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageFile_Impl,
        const OFFSET: isize,
    >() -> IDStorageFile_Vtbl {
        unsafe extern "system" fn Close<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close()
        }
        unsafe extern "system" fn GetFileInformation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageFile_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            info: *mut ::windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(info, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            GetFileInformation: GetFileInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageFile as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageQueue_Impl: Sized {
    fn EnqueueRequest(&self, request: *const DSTORAGE_REQUEST);
    fn EnqueueStatus(&self, statusarray: &::core::option::Option<IDStorageStatusArray>, index: u32);
    fn EnqueueSignal(
        &self,
        fence: &::core::option::Option<::windows::Win32::Graphics::Direct3D12::ID3D12Fence>,
        value: u64,
    );
    fn Submit(&self);
    fn CancelRequestsWithTag(&self, mask: u64, value: u64);
    fn Close(&self);
    fn GetErrorEvent(&self) -> ::windows::Win32::Foundation::HANDLE;
    fn RetrieveErrorRecord(&self, record: *mut DSTORAGE_ERROR_RECORD);
    fn Query(&self, info: *mut DSTORAGE_QUEUE_INFO);
}
impl ::windows::core::RuntimeName for IDStorageQueue {}
impl IDStorageQueue_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageQueue_Impl,
        const OFFSET: isize,
    >() -> IDStorageQueue_Vtbl {
        unsafe extern "system" fn EnqueueRequest<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            request: *const ::core::mem::ManuallyDrop<DSTORAGE_REQUEST>,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueRequest(::core::mem::transmute_copy(&request))
        }
        unsafe extern "system" fn EnqueueStatus<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            statusarray: *mut ::core::ffi::c_void,
            index: u32,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueStatus(
                ::core::mem::transmute(&statusarray),
                ::core::mem::transmute_copy(&index),
            )
        }
        unsafe extern "system" fn EnqueueSignal<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fence: *mut ::core::ffi::c_void,
            value: u64,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueSignal(
                ::core::mem::transmute(&fence),
                ::core::mem::transmute_copy(&value),
            )
        }
        unsafe extern "system" fn Submit<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit()
        }
        unsafe extern "system" fn CancelRequestsWithTag<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            mask: u64,
            value: u64,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelRequestsWithTag(
                ::core::mem::transmute_copy(&mask),
                ::core::mem::transmute_copy(&value),
            )
        }
        unsafe extern "system" fn Close<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close()
        }
        unsafe extern "system" fn GetErrorEvent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::Win32::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorEvent()
        }
        unsafe extern "system" fn RetrieveErrorRecord<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            record: *mut ::core::mem::ManuallyDrop<DSTORAGE_ERROR_RECORD>,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetrieveErrorRecord(::core::mem::transmute_copy(&record))
        }
        unsafe extern "system" fn Query<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            info: *mut ::core::mem::ManuallyDrop<DSTORAGE_QUEUE_INFO>,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Query(::core::mem::transmute_copy(&info))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnqueueRequest: EnqueueRequest::<Identity, Impl, OFFSET>,
            EnqueueStatus: EnqueueStatus::<Identity, Impl, OFFSET>,
            EnqueueSignal: EnqueueSignal::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            CancelRequestsWithTag: CancelRequestsWithTag::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetErrorEvent: GetErrorEvent::<Identity, Impl, OFFSET>,
            RetrieveErrorRecord: RetrieveErrorRecord::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageQueue1_Impl: Sized + IDStorageQueue_Impl {
    fn EnqueueSetEvent(&self, handle: ::windows::Win32::Foundation::HANDLE);
}
impl ::windows::core::RuntimeName for IDStorageQueue1 {}
impl IDStorageQueue1_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageQueue1_Impl,
        const OFFSET: isize,
    >() -> IDStorageQueue1_Vtbl {
        unsafe extern "system" fn EnqueueSetEvent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageQueue1_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: ::windows::Win32::Foundation::HANDLE,
        ) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueSetEvent(::core::mem::transmute_copy(&handle))
        }
        Self {
            base__: IDStorageQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageQueue1 as ::windows::core::Interface>::IID
            || iid == &<IDStorageQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDStorageStatusArray_Impl: Sized {
    fn IsComplete(&self, index: u32) -> bool;
    fn GetHResult(&self, index: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDStorageStatusArray {}
impl IDStorageStatusArray_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDStorageStatusArray_Impl,
        const OFFSET: isize,
    >() -> IDStorageStatusArray_Vtbl {
        unsafe extern "system" fn IsComplete<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageStatusArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
        ) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsComplete(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetHResult<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDStorageStatusArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHResult(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsComplete: IsComplete::<Identity, Impl, OFFSET>,
            GetHResult: GetHResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDStorageStatusArray as ::windows::core::Interface>::IID
    }
}
