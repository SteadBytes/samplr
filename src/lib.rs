use rand::{rngs::StdRng, Rng, SeedableRng};

/// Uniformly sample `n` items from `input` via [Reservoir Sampling](https://steadbytes.com/blog/reservoir-sampling/#algorithm-l-geometric-jumps).
///
// If not `None`, seed the RNG with `seed`.
pub fn reservoir_sample<I, T>(input: I, n: u32, seed: Option<u64>) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    let mut iter = input.into_iter();
    let mut reservoir: Vec<T> = iter.by_ref().take(n as usize).collect();
    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(rand::thread_rng()).unwrap(),
    };
    let mut w: f64 = (rng.gen::<f64>().ln() / n as f64).exp();

    loop {
        let s = (rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() as usize;
        match iter.nth(s + 1) {
            Some(e) => {
                reservoir[rng.gen_range(0..n) as usize] = e;
                w *= (rng.gen::<f64>().ln() / n as f64).exp();
            }
            None => return reservoir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_pop_when_pop_size_less_than_sample_size() {
        // Population size of 10
        let population: Vec<u32> = (0..10).collect();

        // 20 element sample
        let sample = reservoir_sample(population.clone(), 20, None);

        assert_eq!(population, sample);
    }
}

#[cfg(test)]
#[cfg(feature = "statistical_tests")]
mod statistical_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand_distr::{Distribution, Exp};

    /// Statistical test using the Maximum Likelihood Estimator of the sample distribution to test
    /// that the sample is uniformly distribution across the population.
    /// https://steadbytes.com/blog/reservoir-sampling/#how-can-a-reservoir-sampling-implementation-be-practically-tested-for-correctness
    #[test]
    fn sample_distribution_mle() {
        let n: u32 = 1000;
        let pop_size = (n * 1000) as usize;
        let rate = 0.1;
        let tolerance = 0.1;
        let rounds = 5;

        for _ in 0..rounds {
            let population = generate_population(rate, pop_size);

            let sample = reservoir_sample(population, n, None);

            assert_eq!(sample.len(), n as usize);

            let rate_mle: f64 = sample.len() as f64 / sample.into_iter().sum::<f64>();
            assert_approx_eq!(rate_mle, rate, tolerance)
        }
    }

    fn generate_population(rate: f64, size: usize) -> Vec<f64> {
        let rng = rand::thread_rng();
        let exp = Exp::new(rate).unwrap();
        let population = {
            let mut v: Vec<f64> = exp.sample_iter(rng).take(size as usize).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v
        };
        population
    }
}
