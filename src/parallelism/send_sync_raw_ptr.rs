use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct SendSyncRawPtr<T> {
    pub ptr: *mut T,
}

impl<T> SendSyncRawPtr<T> {
    pub unsafe fn set(&self, value: T) {
        unsafe { self.ptr.write(value) };
    }

    pub unsafe fn get(&self) -> T {
        unsafe { self.ptr.read() }
    }
}

unsafe impl<T> Send for SendSyncRawPtr<T> {}
unsafe impl<T> Sync for SendSyncRawPtr<T> {}

impl<T> Clone for SendSyncRawPtr<T> {
    fn clone(&self) -> Self {
        SendSyncRawPtr { ptr: self.ptr }
    }
}

impl<T> Copy for SendSyncRawPtr<T> {}

impl<T> From<*mut T> for SendSyncRawPtr<T> {
    fn from(value: *mut T) -> Self {
        SendSyncRawPtr { ptr: value }
    }
}

impl<T> From<SendSyncRawPtr<T>> for *mut T {
    fn from(value: SendSyncRawPtr<T>) -> Self {
        value.ptr
    }
}

impl<T> Add<usize> for SendSyncRawPtr<T> {
    type Output = SendSyncRawPtr<T>;

    fn add(self, rhs: usize) -> Self::Output {
        SendSyncRawPtr {
            ptr: unsafe { self.ptr.add(rhs) },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parallelism::send_sync_raw_ptr::SendSyncRawPtr;

    #[test]
    fn test_set() {
        let mut nums = (0..5).collect::<Vec<i32>>();
        let ptr = SendSyncRawPtr {
            ptr: nums.as_mut_ptr(),
        } + 3;

        unsafe { ptr.set(69) };

        assert_eq!(nums, vec![0, 1, 2, 69, 4]);
    }

    #[test]
    fn test_get() {
        let mut nums = (0..5).collect::<Vec<i32>>();
        let ptr = SendSyncRawPtr {
            ptr: nums.as_mut_ptr(),
        } + 3;

        let third = unsafe { ptr.get() };

        assert_eq!(third, 3);
    }
}
