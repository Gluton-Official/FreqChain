use nih_plug::buffer::{Buffer, ChannelSamples, ChannelSamplesIter};

pub trait BufferUtils {
    /// ### Arguments for F
    /// - `channel`
    /// - `sample_index`
    /// - `sample`
    fn on_each_sample<F>(&mut self, f: F)
    where
        F: FnMut(usize, usize, &mut f32);
}

impl BufferUtils for Buffer<'_> {
    fn on_each_sample<F>(&mut self, mut f: F)
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
