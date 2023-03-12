mod complex;
mod digital_signal_processing;

pub fn main() {
    let sz = 1048576;
    let val = digital_signal_processing::fft::<f64>(
        &(0..sz).into_iter().map(|x| x as f64).collect::<Vec<f64>>(),
    );
    let orig: Vec<f64> = digital_signal_processing::ifft(&val);
}
