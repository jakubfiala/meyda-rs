use extractors::amp_spectrum as amp_spectrum;

pub fn compute(signal : &Vec<f64>) -> Vec<f64> {
  let amp_spec : Vec<f64> = amp_spectrum::compute(signal);

  let pow_spec = amp_spec.iter().map(|bin| bin * bin).into_iter().collect();

  return pow_spec;
}