use nih_plug::buffer::Buffer;

pub trait BufferUtils<T> {
    /// ### Arguments for F
    /// - `channel`
    /// - `index`
    /// - `data`
    fn on_each<F>(&mut self, f: F)
    where
        F: FnMut(usize, usize, &mut T);

    /// ### Arguments for F
    /// - `index`
    /// - `data`
    fn on_each_by_channel<F>(&mut self, channel: usize, mut f: F)
    where
        F: FnMut(usize, &mut T)
    {
        self.on_each(|channel_index, index, data| {
            if channel_index == channel {
                f(index, data);
            }
        })
    }
}

impl BufferUtils<f32> for Buffer<'_> {
    fn on_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut f32)
    {
        for (sample_index, channel_samples) in self.iter_samples().enumerate() {
            for (channel, sample) in channel_samples.into_iter().enumerate() {
                f(channel, sample_index, sample);
            }
        }
    }
}

impl<T> BufferUtils<T> for Vec<Vec<T>> {
    fn on_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut T)
    {
        for (channel_index, channel_buffer) in self.iter_mut().enumerate() {
            for (bin_index, bin) in channel_buffer.iter_mut().enumerate() {
                f(channel_index, bin_index, bin);
            }
        }
    }
    
    fn on_each_by_channel<F>(&mut self, channel: usize, mut f: F)
    where
        F: FnMut(usize, &mut T)
    {
        for (bin_index, bin) in self[channel].iter_mut().enumerate() {
            f(bin_index, bin);
        };
    }
}