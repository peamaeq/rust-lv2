use crate::port::{InPlacePortType, PortType};
use crate::port_collection::PortCollection;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct InputOutputPort<T: PortType> {
    input: NonNull<c_void>,
    output: NonNull<c_void>,
    sample_count: u32,
    _port: PhantomData<T>,
}

impl<T: PortType> InputOutputPort<T> {
    pub fn input(&self) -> T::Input {
        // SAFETY: Pointer validity is upheld by from_connections, and is the only way to construct this type.
        unsafe { T::input_from_raw(self.input, self.sample_count) }
    }

    pub fn output(&mut self) -> T::Output {
        // SAFETY: Pointer validity is upheld by from_connections, and is the only way to construct this type.
        unsafe { T::output_from_raw(self.input, self.sample_count) }
    }

    /* pub fn input_output(&mut self) -> (&[Cell<f32>], &[Cell<f32>]) {
        todo!()
    }*/

    /*pub fn zip(&mut self) -> impl Iterator<Item = (&Cell<f32>, &Cell<f32>)> {
        todo!()
    }*/
}

impl<T: InPlacePortType> InputOutputPort<T> {
    #[inline]
    pub fn input_output(&mut self) -> T::InputOutput {
        // SAFETY: Pointer validity is upheld by from_connections, and is the only way to construct this type.
        unsafe { T::input_output_from_raw(self.input, self.output, self.sample_count) }
    }
}

impl<I: Sized + 'static, T: InPlacePortType<InputOutput = (&'static [I], &'static [I])>>
    InputOutputPort<T>
{
    #[inline]
    pub fn zip(&mut self) -> impl Iterator<Item = (&I, &I)> {
        let (input, output) = self.input_output();
        input.iter().zip(output)
    }
}

impl<T: PortType> PortCollection for InputOutputPort<T> {
    type Connections = [*mut c_void; 2];

    unsafe fn from_connections(cache: &Self::Connections, sample_count: u32) -> Option<Self> {
        Some(Self {
            input: NonNull::new(cache[0])?,
            output: NonNull::new(cache[1])?,
            sample_count,
            _port: PhantomData,
        })
    }
}
