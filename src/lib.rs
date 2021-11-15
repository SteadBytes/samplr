use rand::Rng;

pub fn sample<I, T>(input: I, n: u32) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    let mut iter = input.into_iter();
    let mut reservoir: Vec<T> = iter.by_ref().take(n as usize).collect();
    let mut rng = rand::thread_rng();
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
    use assert_approx_eq::assert_approx_eq;
    use rand_distr::{Distribution, Exp};

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

    #[test]
    fn sample_distribution() {
        let n: u32 = 1000;
        let pop_size = (n * 1000) as usize;
        let rate = 0.1;
        let tolerance = 0.1;
        let rounds = 5;

        for _ in 0..rounds {
            let population = generate_population(rate, pop_size);

            let sample = sample(population, n);

            let rate_mle: f64 = sample.len() as f64 / sample.into_iter().sum::<f64>();
            assert_approx_eq!(rate_mle, rate, tolerance)
        }
    }
}
