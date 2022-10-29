// Attempting to make a filter....
//
use realfft::RealFftPlanner;

pub trait Filter {
    fn process(&self, buffer: &mut Vec<f32>);
}

pub struct SimpleFilter {
    frequency: f32,
    sample_rate: f32,
}

impl SimpleFilter {
    pub fn new(frequency_norm: f32, sample_rate: f32) -> Self {
        // parameters are 0 < p < 1, so we need to scale this
        // to frequencies. the max frequency is half of the
        // sample rate.
        let frequency = frequency_norm * (sample_rate / 2.);
        SimpleFilter {
            frequency,
            // can't store frequency_per_bucket here
            // because it depends on the buffer size.
            sample_rate,
        }
    }
}

impl Filter for SimpleFilter {
    fn process(&self, buffer: &mut Vec<f32>) {
        let buffer_size = buffer.len();

        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(buffer_size);

        let mut spectrum = fft.make_output_vec();
        let spectrum_size = spectrum.len();

        // process forwards
        fft.process(buffer, &mut spectrum).unwrap();

        // find cutoff bucket
        let frequency_per_bucket = (self.sample_rate / 2.0) / (spectrum_size as f32);
        let cutoff_bucket_index = (self.frequency / frequency_per_bucket) as usize;

        // because this is a dumb filter, just zero out all the buckets over the cutoff
        for i in 0..spectrum_size {
            // we need to normalize here or the amplitudes of our output
            // samples will be wrong
            spectrum[i] = spectrum[i].unscale(buffer_size as f32);

            if i >= cutoff_bucket_index {
                // TODO: come up with some other scale factor
                // it is pretty tricky to get this to work tho.
                spectrum[i] = spectrum[i].scale(0.0);
            }
        }

        let ffti = planner.plan_fft_inverse(buffer_size);
        ffti.process(&mut spectrum, buffer).unwrap();
    }
}
