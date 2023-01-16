use std::{ffi::c_void, mem::ManuallyDrop};

use microsoft_directstorage::{
    DStorageGetFactory, IDStorageFactory, IDStorageFile, IDStorageQueue, DSTORAGE_DESTINATION,
    DSTORAGE_DESTINATION_BUFFER, DSTORAGE_ERROR_RECORD, DSTORAGE_MAX_QUEUE_CAPACITY,
    DSTORAGE_PRIORITY_NORMAL, DSTORAGE_QUEUE_DESC, DSTORAGE_REQUEST, DSTORAGE_REQUEST_OPTIONS,
    DSTORAGE_REQUEST_SOURCE_FILE, DSTORAGE_SOURCE, DSTORAGE_SOURCE_FILE,
};
use windows::{
    core::{Interface, Vtable},
    w,
    Win32::{
        Foundation::{WAIT_OBJECT_0, WIN32_ERROR},
        Graphics::{
            Direct3D::D3D_FEATURE_LEVEL_12_1,
            Direct3D12::{
                D3D12CreateDevice, ID3D12Device, ID3D12Fence, ID3D12Resource,
                D3D12_FENCE_FLAG_NONE, D3D12_HEAP_FLAG_NONE, D3D12_HEAP_PROPERTIES,
                D3D12_HEAP_TYPE_DEFAULT, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_BUFFER,
                D3D12_RESOURCE_STATE_COMMON, D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            },
            Dxgi::Common::{DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC},
        },
        System::{
            Threading::{CreateEventW, WaitForSingleObject},
            WindowsProgramming::INFINITE,
        },
    },
};

//

use std::sync::Arc;
use downcast_rs::{impl_downcast, Downcast, DowncastSync};

trait File: Downcast {}
impl_downcast!(File);

struct StorageSource {
    file: Arc<dyn File>,
    offset_in_bytes: usize,
    size_in_bytes: usize,
}

enum StorageDestination {
    Buffer {
        resource: ID3D12Resource,
        offset_in_bytes: usize,
    },
    Texture
}

struct StorageRequest {
    source: StorageSource,
    destination: StorageDestination,
    uncompressed_size_in_bytes: usize,
}

trait Queue {
    fn enqueue_request(&self, request: StorageRequest);
    fn submit();
}

struct DirectStorageFile {
    handle: IDStorageFile
}

impl DirectStorageFile {
    fn new(factory: &IDStorageFactory, file_name: &std::path::Path) -> Self {
        use std::os::windows::prelude::*;

        let file_name = ::windows::core::HSTRING::from(file_name.as_os_str());
        let handle: IDStorageFile = unsafe { factory.OpenFile(&file_name).unwrap() };

        Self {
            handle
        }
    }
}
impl File for DirectStorageFile {}

struct DirectStorageQueue {
    handle: IDStorageQueue,
}

impl DirectStorageQueue {
    fn new(factory: &IDStorageFactory, device: ID3D12Device) -> Self {
        let queue_description = DSTORAGE_QUEUE_DESC {
            SourceType: DSTORAGE_REQUEST_SOURCE_FILE,
            Capacity: DSTORAGE_MAX_QUEUE_CAPACITY as u16,
            Priority: DSTORAGE_PRIORITY_NORMAL,
            Device: Some(device.clone()),
            ..Default::default()
        };

        let handle: IDStorageQueue = unsafe { factory.CreateQueue(&queue_description).unwrap() };
    
        Self {
            handle
        }
    }
}

impl Queue for DirectStorageQueue {
    fn enqueue_request(&self, request: StorageRequest) {
        let source_file = request.source.file.downcast_ref::<DirectStorageFile>().unwrap();

        let source_file = ManuallyDrop::new(DSTORAGE_SOURCE_FILE {
            Offset: request.source.offset_in_bytes.try_into().unwrap(),
            Size: request.source.size_in_bytes.try_into().unwrap(),
            Source: Some(source_file.handle.clone()),
        });

        let destination = match request.destination {
            StorageDestination::Buffer { resource, offset_in_bytes } => {
                let destination_buffer = ManuallyDrop::new(DSTORAGE_DESTINATION_BUFFER {
                    Resource: Some(resource),
                    Offset: offset_in_bytes.try_into().unwrap(),
                    Size: request.source.size_in_bytes.try_into().unwrap(),
                });
                
                DSTORAGE_DESTINATION {
                    Buffer: destination_buffer,
                }
            },
            _ => unimplemented!()
        };

        let dsrequest = DSTORAGE_REQUEST {
            Options: DSTORAGE_REQUEST_OPTIONS {
                // BUG: Metadata
                Upper: 0,
                Lower: 2,
            },
            Source: DSTORAGE_SOURCE { File: source_file },
            Destination: destination,
            UncompressedSize: request.uncompressed_size_in_bytes.try_into().unwrap(),
            ..Default::default()
        };

        unsafe { self.handle.EnqueueRequest(&dsrequest) };    
    }
}

trait StorageDevice {
    fn open_file(&self, file_name: impl AsRef<std::path::Path>) -> Arc<dyn File>;
    fn create_queue(&self, device: ID3D12Device) -> Arc<dyn Queue>;
}

struct DirectStorageDevice {
    factory: IDStorageFactory,
}

impl DirectStorageDevice {
    fn new() -> Self {
        let mut factory: *mut c_void = std::ptr::null_mut();
        unsafe { DStorageGetFactory(&IDStorageFactory::IID, &mut factory).unwrap() };
        let factory = unsafe { IDStorageFactory::from_raw(factory) };
    
        Self {
            factory
        }
    }
}

impl StorageDevice for DirectStorageDevice {
    fn open_file(&self, file_name: impl AsRef<std::path::Path>) -> Arc<dyn File> {
        Arc::new(DirectStorageFile::new(
            &self.factory,
            file_name.as_ref(),
        ))
    }

    fn create_queue(&self, device: ID3D12Device) -> Arc<dyn Queue> {
        Arc::new(DirectStorageQueue::new(
            &self.factory,
            device
        ))
    }
}

fn main() -> windows::core::Result<()> {
    //
    // Create virtual adapter and DirectStorage factory
    //

    let mut device: Option<ID3D12Device> = None;
    unsafe { D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_1, &mut device)? };
    let device = device.unwrap();

    let mut factory: *mut c_void = std::ptr::null_mut();
    unsafe { DStorageGetFactory(&IDStorageFactory::IID, &mut factory)? };
    let factory = unsafe { IDStorageFactory::from_raw_borrowed(&factory) };

    let dsdevice = DirectStorageDevice::new();
    dsdevice.create_queue(device.clone());

    //
    // Prepare file input
    //

    std::fs::write("directstorage_sample.txt", b"Hello DirectStorage!").expect("Failed to write to directstorage.sample.txt.");

    let file: IDStorageFile = unsafe { factory.OpenFile(w!(r#"directstorage_sample.txt"#))? };
    let file_info = unsafe { file.GetFileInformation()? };
    let file_size = file_info.nFileSizeLow;

    //
    // Create a DirectStorage queue to load data onto the GPU
    //

    let queue_description = DSTORAGE_QUEUE_DESC {
        SourceType: DSTORAGE_REQUEST_SOURCE_FILE, // BUG: Metadata
        Capacity: DSTORAGE_MAX_QUEUE_CAPACITY as u16,
        Priority: DSTORAGE_PRIORITY_NORMAL,
        Device: Some(device.clone()),
        ..Default::default()
    };
    let queue: IDStorageQueue = unsafe { factory.CreateQueue(&queue_description)? };

    //
    // Create resource and backing heap
    //

    let heap_properties = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_DEFAULT,
        ..Default::default()
    };
    let buffer_descriptor = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Width: file_size as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_UNKNOWN,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        ..Default::default()
    };
    let mut resource: Option<ID3D12Resource> = None;
    unsafe {
        device.CreateCommittedResource(
            &heap_properties,
            D3D12_HEAP_FLAG_NONE,
            &buffer_descriptor,
            D3D12_RESOURCE_STATE_COMMON,
            None,
            &mut resource,
        )?
    };

    //
    // Enqueue a file read request
    //

    let source_file = ManuallyDrop::new(DSTORAGE_SOURCE_FILE {
        Offset: 0,
        Size: file_size,
        Source: Some(file),
    });
    let destination_buffer = ManuallyDrop::new(DSTORAGE_DESTINATION_BUFFER {
        Resource: resource,
        Offset: 0,
        Size: file_size,
    });
    let request = DSTORAGE_REQUEST {
        Options: DSTORAGE_REQUEST_OPTIONS {
            // BUG: Metadata
            Upper: 0,
            Lower: 2,
        },
        Source: DSTORAGE_SOURCE { File: source_file },
        Destination: DSTORAGE_DESTINATION {
            Buffer: destination_buffer,
        },
        UncompressedSize: file_size,
        ..Default::default()
    };
    unsafe { queue.EnqueueRequest(&request) };

    //
    // Create a fence (an object used for synchronization of the CPU and one or more GPUs)
    // and enqueue a fence write to signal the completion of the read request.
    //

    let fence: ID3D12Fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE)? };
    let fence_value = 1;
    let event_handle = unsafe { CreateEventW(None, false, false, None)? };
    unsafe {
        fence.SetEventOnCompletion(fence_value, event_handle)?;
        queue.EnqueueSignal(&fence, fence_value);
        queue.Submit();
    }

    //
    // Wait for completion signal
    //

    println!("Waiting for DirectStorage request to complete...");
    match unsafe { WaitForSingleObject(event_handle, INFINITE) } {
        WAIT_OBJECT_0 => {}
        WIN32_ERROR(error) => panic!("Wait failed. {error}"),
    };

    //
    // Check for errors
    //

    let mut error_record = DSTORAGE_ERROR_RECORD::default();
    unsafe { queue.RetrieveErrorRecord(&mut error_record) };

    if error_record.FailureCount > 0 {
        println!(
            "DirectStorage request failed, first failure: {}",
            error_record.FirstFailure.HResult
        );
    } else {
        println!("DirectStorage request completed successfully!");
    }

    Ok(())
}
