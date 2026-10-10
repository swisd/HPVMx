use crate::neural::network::Network;
use alloc::vec;

/// Converts normalized RGB [0.0, 1.0] to YCoCg (Luminance, Chrominance Orange, Chrominance Green).
/// All outputs are perfectly bound between [0.0, 1.0].
#[inline(always)]
pub fn rgb_to_ycocg(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // 1. Calculate raw YCoCg
    let y = 0.25 * r + 0.5 * g + 0.25 * b; // Luminance (0.0 to 1.0)
    let co = 0.5 * r - 0.5 * b; // Chrominance Orange (-0.5 to 0.5)
    let cg = -0.25 * r + 0.5 * g - 0.25 * b; // Chrominance Green (-0.5 to 0.5)

    // 2. Normalize Chrominance channels from [-0.5, 0.5] to [0.0, 1.0]
    let co_norm = co + 0.5;
    let cg_norm = cg + 0.5;

    (y, co_norm, cg_norm)
}

/// Converts normalized YCoCg back to standard [0.0, 1.0] RGB.
#[inline(always)]
pub fn ycocg_to_rgb(y: f32, co_norm: f32, cg_norm: f32) -> (f32, f32, f32) {
    // 1. Denormalize chrominance channels back to [-0.5, 0.5]
    let co = co_norm - 0.5;
    let cg = cg_norm - 0.5;

    // 2. Reconstruct RGB
    let tmp = y - cg;
    let r = tmp + co;
    let g = y + cg;
    let b = tmp - co;

    // 3. Clamp values to ensure standard boundaries
    (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
}

fn test() {
    let mut net = Network::new(3, &[4, 4], 2);

    // Mock incoming normalized RGB pixel data
    let (r, g, b) = (0.8, 0.2, 0.3);

    // 1. Convert to Luminance and Chrominance
    let (lum, chrom_o, chrom_g) = rgb_to_ycocg(r, g, b);

    // 2. Format into your network's input structure (attaching default weights of 1.0)
    let network_input = vec![
        ((lum, 1.0), 0),     // Neuron 0: Luminance
        ((chrom_o, 1.0), 1), // Neuron 1: Chrominance Orange
        ((chrom_g, 1.0), 2), // Neuron 2: Chrominance Green
    ];

    // 3. Run the forward pass
    let final_output = net.feed_forward(network_input);
}
