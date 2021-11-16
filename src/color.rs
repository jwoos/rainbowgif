use palette::rgb::LinSrgba;
use palette::Gradient;
use std::vec;

pub struct GradientDescriptor {
    colors: vec::Vec<LinSrgba<f64>>,
    positions: vec::Vec<f64>,
    wrap: bool,
}

struct KeyFrame {
    color: LinSrgba<f64>,
}

impl GradientDescriptor {
    pub fn new(colors: vec::Vec<LinSrgba<f64>>, wrap: bool) -> GradientDescriptor {
        let rng = if wrap && colors.len() > 1 {
            0..colors.len() + 1
        } else {
            0..colors.len()
        };
        let length = colors.len();
        return GradientDescriptor {
            colors,
            positions: rng.map(|i| (i as f64) / (length as f64)).collect(),
            wrap,
        };
    }

    pub fn generate(self, frame_count: usize) -> vec::Vec<LinSrgba<f64>> {
        let mut gen = vec::Vec::<LinSrgba<f64>>::new();

        // this only counts the number of colors forward, excluding color itself and the
        // destination
        let frames_per_color = frame_count / (self.colors.len() + 1);
        let mut base_index = 0;
        for (i, color) in self.colors.iter().enumerate() {
            let src = color;
            let dest = if i == self.colors.len() - 1 {
                &self.colors[0]
            } else {
                &self.colors[i + 1]
            };
            let grad_iter = Gradient::new(vec![src.clone(), dest.clone()]);

            println!("{}", frames_per_color);
            println!(
                "src: {} {} {}",
                src.color.red, src.color.green, src.color.blue
            );
            println!(
                "dest: {} {} {}",
                dest.color.red, dest.color.green, dest.color.blue
            );

            let colors = grad_iter.take(frames_per_color + 2).collect::<Vec<_>>();
            for j in 0..frames_per_color + 1 {
                println!(
                    "{}: {} {} {}",
                    i + j,
                    colors[j].color.red,
                    colors[j].color.green,
                    colors[j].color.blue
                );
                gen.push(colors[j]);
            }
        }

        return gen;
    }
}

mod tests {
    use palette::rgb::LinSrgba;

    use crate::color;

    #[test]
    fn test_new() {
        let grad_desc = color::GradientDescriptor::new(
            vec![
                LinSrgba::new(0., 0., 0., 1.),
                LinSrgba::new(0.5, 0.5, 0.5, 1.),
                LinSrgba::new(1., 1., 1., 1.),
            ],
            true,
        );

        let colors = grad_desc.generate(12);

        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].alpha, 1.0);
        assert_eq!(colors[0].color.red, 0.0);
        assert_eq!(colors[0].color.green, 0.0);
        assert_eq!(colors[0].color.blue, 0.0);

        assert_eq!(colors[4].alpha, 1.0);
        assert_eq!(colors[4].color.red, 0.5);
        assert_eq!(colors[4].color.green, 0.5);
        assert_eq!(colors[4].color.blue, 0.5);

        assert_eq!(colors[8].alpha, 1.0);
        assert_eq!(colors[8].color.red, 1.0);
        assert_eq!(colors[8].color.green, 1.0);
        assert_eq!(colors[8].color.blue, 1.0);
    }
}
