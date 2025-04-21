use std::sync::Arc;

use realfft::num_complex::Complex;
use realfft::num_traits::Zero;
use realfft::ComplexToReal;
use realfft::FftNum;
use realfft::RealFftPlanner;
use realfft::RealToComplex;

pub const LENGTH: usize = 1024;
pub const WINDOW_OVERLAP: usize = 2;
pub const BUFFER_SIZE: usize = LENGTH / 2 + 1;

// TODO: join into one struct
pub trait FFT<T: FftNum> {
    fn new(planner: &mut RealFftPlanner<T>, length: usize) -> Self;
    fn length(&self) -> usize;
    fn create_real_buffer(&self) -> Vec<T>;
    fn create_complex_buffer(&self) -> Vec<Complex<T>>;
}

pub struct ForwardFFT<T: FftNum> {
    plan: Arc<dyn RealToComplex<T>>,
    length: usize,
}

impl<T: FftNum> ForwardFFT<T> {
    pub fn process(&self, input: &mut [T], output: &mut [Complex<T>]) {
        self.plan.process_with_scratch(input, output, &mut []).unwrap()
    }
}

pub struct InverseFFT<T: FftNum> {
    plan: Arc<dyn ComplexToReal<T>>,
    length: usize,
}

impl<T: FftNum> InverseFFT<T> {
    pub fn process(&self, input: &mut [Complex<T>], output: &mut [T]) {
        self.plan.process_with_scratch(input, output, &mut []).unwrap()
    }
}

impl<T: FftNum> FFT<T> for ForwardFFT<T> {
    fn new(planner: &mut RealFftPlanner<T>, length: usize) -> Self {
        Self {
            plan: planner.plan_fft_forward(length),
            length,
        }
    }

    fn length(&self) -> usize {
        self.length
    }

    fn create_real_buffer(&self) -> Vec<T> {
        self.plan.make_input_vec()
    }

    fn create_complex_buffer(&self) -> Vec<Complex<T>> {
        self.plan.make_output_vec()
    }
}

impl<T: FftNum> FFT<T> for InverseFFT<T> {
    fn new(planner: &mut RealFftPlanner<T>, length: usize) -> Self {
        Self {
            plan: planner.plan_fft_inverse(length),
            length,
        }
    }

    fn length(&self) -> usize {
        self.length
    }

    fn create_real_buffer(&self) -> Vec<T> {
        self.plan.make_output_vec()
    }

    fn create_complex_buffer(&self) -> Vec<Complex<T>> {
        self.plan.make_input_vec()
    }
}

pub fn create_fft_pair<T: FftNum>(length: usize) -> (ForwardFFT<T>, InverseFFT<T>) {
    let mut planner = RealFftPlanner::new();
    (
        ForwardFFT::new(&mut planner, length),
        InverseFFT::new(&mut planner, length),
    )
}

pub fn create_real_buffer<T: FftNum>(length: usize) -> Vec<T> {
    vec![T::zero(); length]
}

pub fn create_complex_buffer<T: FftNum>(length: usize) -> Vec<Complex<T>> {
    vec![Complex::zero(); length / 2 + 1]
}
