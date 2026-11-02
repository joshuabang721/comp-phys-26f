use rand::rngs::ThreadRng;

pub fn homework6(_rng: &mut ThreadRng) -> () {}

fn _hw6_question1(_rng: &mut ThreadRng) -> () {}

fn hebbian_interaction_j_i_j(memory_x_mu_i: Vec<Vec<f64>>, n: usize) -> Vec<Vec<f64>> {
    let x_mu_i = memory_x_mu_i;
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        x_mu_i.iter().map(|x_i| x_i[i] * x_i[j]).sum::<f64>() / (n as f64)
                    }
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

fn energy_e(interaction_j_i_k: Vec<Vec<f64>>, neurons_s_i: Vec<i32>) -> f64 {
    let j_i_k = interaction_j_i_k;
    let s_i = neurons_s_i;
    -0.5 * j_i_k
        .iter()
        .enumerate()
        .map(|(i, j_k)| {
            j_k.iter()
                .enumerate()
                .map(|(k, j)| j * (s_i[i] as f64) * (s_i[k] as f64))
                .sum::<f64>()
        })
        .sum::<f64>()
}

fn local_field_h_i(interaction_j_i_k: Vec<Vec<f64>>, neurons_s_i: Vec<i32>) -> Vec<f64> {
    let j_i_k = interaction_j_i_k;
    let s_i = neurons_s_i;
    j_i_k
        .iter()
        .map(|j_k| {
            j_k.iter()
                .enumerate()
                .map(|(k, j)| j * (s_i[k] as f64))
                .sum::<f64>()
        })
        .collect::<Vec<f64>>()
}

fn glauber_update_p_i(local_field_h_i: Vec<f64>, temperature_t: f64) -> Vec<f64> {
    let h_i = local_field_h_i;
    let t = temperature_t;
    h_i.iter()
        .map(|h| 1.0 / (1.0 + (-2.0 * h / t).exp()))
        .collect::<Vec<f64>>()
}

fn retrieval_overlap_m1(memory_x_1_i: Vec<f64>, neurons_s_i: Vec<i32>) -> f64 {
    let iter = std::iter::zip(memory_x_1_i, neurons_s_i);
    let n = iter.len();
    iter.map(|(x_1_i, s_i)| x_1_i * (s_i as f64)).sum::<f64>() / (n as f64)
}

fn late_time_average_avgmbar(
    retrieval_overlap_m1_r_t: Vec<Vec<f64>>,
    initial_transient_tb: usize,
) -> f64 {
    let m1_r_t = retrieval_overlap_m1_r_t;
    let tb = initial_transient_tb;
    let r = m1_r_t.len();

    m1_r_t
        .iter()
        .map(|m1_t| {
            let iter = m1_t.iter().skip(tb);
            let t = iter.len();
            iter.sum::<f64>() / (t as f64)
        })
        .sum::<f64>()
        / (r as f64)
}
