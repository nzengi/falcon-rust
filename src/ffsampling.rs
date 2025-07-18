// Falcon için FFT tabanlı örnekleme ve Gram matrisi (ffsampling.py'den çevrildi)

pub fn gram(b: &[Vec<Vec<f64>>]) -> Vec<Vec<Vec<f64>>> {
    let rows = b.len();
    let ncols = b[0].len();
    let deg = b[0][0].len();
    let mut g = vec![vec![vec![0.0; deg]; rows]; rows];
    for i in 0..rows {
        for j in 0..rows {
            for k in 0..ncols {
                for coef in 0..deg {
                    g[i][j][coef] += b[i][k][coef] * b[j][k][coef]; // adj fonksiyonu gerekirse eklenir
                }
            }
        }
    }
    g
}
// Diğer fonksiyonlar (ldl, ldl_fft, ffldl, ffldl_fft, ffnp, ffnp_fft, ffsampling_fft) gerektiğinde eklenebilir. 