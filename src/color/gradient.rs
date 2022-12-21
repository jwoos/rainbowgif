use std::vec;

use clap::{builder::PossibleValue, ValueEnum};
use palette::gradient;
use palette::Mix;

use super::{Color, ScalarType};
use crate::commandline;

commandline::define_cli_enum!(GradientGeneratorType, {
    Discrete: ("discrete", "Where the colors are calculated by global and local position"),
    Continuous: ("continuous", "where palette generates it, taking into account all colors"),
});

struct GradientKeyFrame<'a, C>
where
    C: Mix<Scalar = ScalarType> + Sized,
{
    color: &'a C,
    position: ScalarType,
}

pub struct GradientDescriptor<C> {
    pub colors: vec::Vec<C>,
    pub positions: vec::Vec<ScalarType>,
}

impl<C> GradientDescriptor<C>
where
    C: Mix<Scalar = ScalarType> + Color,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<ScalarType>>::Color>,
{
    pub fn new(mut colors: vec::Vec<C>) -> GradientDescriptor<C> {
        colors.push(colors[0].clone());
        let rng = 0..colors.len();
        let length = std::cmp::max(colors.len() - 1, 1);
        return GradientDescriptor {
            colors,
            positions: rng
                .map(|i| (i as ScalarType) / (length as ScalarType))
                .collect(),
        };
    }

    pub fn generate(
        &self,
        frame_count: usize,
        generator_type: GradientGeneratorType,
    ) -> vec::Vec<C> {
        return match generator_type {
            GradientGeneratorType::Continuous => self.generate_continuous(frame_count),
            GradientGeneratorType::Discrete => self.generate_discrete(frame_count),
        };
    }

    fn generate_continuous(&self, frame_count: usize) -> vec::Vec<C> {
        let grad = gradient::Gradient::new(self.colors.clone());
        return grad.take(frame_count + 1).take(frame_count).collect();
    }

    fn generate_discrete(&self, frame_count: usize) -> vec::Vec<C> {
        let mut gen = vec::Vec::<C>::new();

        for i in 0..frame_count {
            let global_position = (i as ScalarType) / (frame_count as ScalarType);

            let (key_frame_src, key_frame_dest) = self.position_search(global_position);
            let local_position = (global_position - key_frame_src.position)
                / (key_frame_dest.position - key_frame_src.position);

            let src = key_frame_src.color;
            let dest = key_frame_dest.color;

            gen.push(src.mix(&dest, local_position));
        }

        return gen;
    }

    fn position_search<'a>(
        &'a self,
        position: ScalarType,
    ) -> (GradientKeyFrame<'a, C>, GradientKeyFrame<'a, C>) {
        let length = self.colors.len() - 1;
        let base = 1.0 / (length as ScalarType);
        let lower_index = (position / base).floor() as usize;

        if lower_index == length {
            return (
                GradientKeyFrame {
                    color: &self.colors[lower_index],
                    position: self.positions[lower_index],
                },
                GradientKeyFrame {
                    color: &self.colors[0],
                    position: self.positions[0],
                },
            );
        }

        return (
            GradientKeyFrame {
                color: &self.colors[lower_index],
                position: self.positions[lower_index],
            },
            GradientKeyFrame {
                color: &self.colors[lower_index + 1],
                position: self.positions[lower_index + 1],
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use palette::{FromColor, Lcha};

    use crate::color;

    #[test]
    fn test_generate_discrete() {
        let grad_desc = color::gradient::GradientDescriptor::new(vec![
            Lcha::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lcha::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lcha::from_color(color::ColorType::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12, color::gradient::GradientGeneratorType::Discrete);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }

    #[test]
    fn test_generate_continuous() {
        let grad_desc = color::gradient::GradientDescriptor::new(vec![
            Lcha::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lcha::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lcha::from_color(color::ColorType::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12, color::gradient::GradientGeneratorType::Continuous);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }
}
