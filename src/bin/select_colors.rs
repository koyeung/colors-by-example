use anyhow::Result;
use clap::{Args, Parser};
use log::{debug, info, warn};
use owo_colors::{AnsiColors, DynColors, OwoColorize, XtermColors};

use std::io::{self, Write};

use colors_by_example::ansi256::Ansi256Colors;
use colors_by_example::ansi256_quantize::Ansi256QuantizedColors;
use colors_by_example::color::Gray;

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[command(flatten)]
    filter: ColorFilter,

    /// Show brighter (true) or darker (false) than reference lightness additionally
    #[arg(long, requires = "lightness")]
    brighten: Option<bool>,

    /// Show color of higher (true)/ lower (false) than reference chroma additionally
    #[arg(long, requires = "chroma")]
    saturate: Option<bool>,

    /// Select opposite hue to the reference one
    #[arg(long, requires = "hue")]
    opposite: bool,

    /// Extent from reference chroma to be considered as similar color
    /// (Chroma are quantized as 255 levels)
    #[arg(long, requires = "chroma", default_value_t = 1)]
    chroma_extent: u8,

    /// Extent from reference hue to be considered as similar color
    /// (Hue are quantized as 255 levels)
    #[arg(long, requires = "hue", default_value_t = 1)]
    hue_extent: u8,

    /// Sample sentence
    #[arg(long, default_value_t=String::from("The Quick Brown Fox Jumps Over the Lazy Dog"))]
    sample: String,

    /// Whether to show sample sentence using selected colors as background
    #[arg(long)]
    background: bool,

    /// Force using color index as fore/background when selected colors are used as back/foreground
    #[arg(long, value_name = "ALT_COLOR")]
    alt: Option<u8>,
}

// Organize filtering arguments in separate groups than other arguments
#[derive(Debug, Args)]
#[group(required = true, multiple = true)]
struct ColorFilter {
    /// Color index for reference lightness
    #[arg(long, short = 'L', value_name = "LIGHTNESS_COLOR")]
    lightness: Option<u8>,

    /// Color index for reference hue
    #[arg(long, short = 'H', value_name = "HUE_COLOR")]
    hue: Option<u8>,

    /// Color index for reference chroma
    #[arg(long, short = 'C', value_name = "CHROMA_COLOR")]
    chroma: Option<u8>,

    /// All gray colors
    #[arg(long, short = 'G')]
    gray: bool,

    /// Base16 (4-bit) colors
    #[arg(long)]
    base16: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("cli options: {:?}", cli);

    let ansi256_colors = Ansi256Colors::default();
    let quantized_colors = Ansi256QuantizedColors::new(&ansi256_colors);

    // ANSI-256 color indexes
    let indexes = select_color_indexes_from_cli(&cli, &quantized_colors);

    dump(
        &indexes,
        &cli.sample,
        cli.background,
        cli.alt,
        &ansi256_colors,
    )?;

    Ok(())
}

fn select_color_indexes_from_cli(cli: &Cli, colors: &Ansi256QuantizedColors) -> Vec<u8> {
    let mut indexes = (0_u8..=255).collect::<Vec<_>>();

    if cli.filter.base16 {
        info!("filter base16 colors");
        indexes.retain(|&i| i < 16);
    }

    if cli.filter.gray {
        info!("filter gray colors");
        indexes = filter_grays(indexes, colors);
    }

    if let Some(ref_index) = cli.filter.lightness {
        let ref_color = colors.color_from_index(ref_index);
        let level = ref_color.lightness_level;

        info!("filter by reference color: #{ref_index}, lightness level: {level}");
        indexes = filter_by_lightness(indexes, level, cli.brighten, colors);
    }

    if let Some(ref_index) = cli.filter.chroma {
        let ref_color = colors.color_from_index(ref_index);
        let level = ref_color.chroma;

        info!(
            "filter by reference color: #{ref_index}, chroma level: {level}: extent: {}",
            cli.chroma_extent
        );
        indexes = filter_by_chroma(indexes, level, cli.chroma_extent, cli.saturate, colors);
    }

    if let Some(ref_index) = cli.filter.hue {
        if !colors.is_gray_from_index(ref_index) {
            // keep gray in hue filtering result might not be desirable
            info!("remove gray from result if filtering using hue");
            indexes.retain(|&i| !colors.is_gray_from_index(i));

            let ref_color = colors.color_from_index(ref_index);
            let level = ref_color.hue;

            info!(
                "filter by reference color: #{ref_index}, hue level: {level}: extent: {}",
                cli.hue_extent
            );
            indexes = filter_by_hue(indexes, level, cli.hue_extent, cli.opposite, colors);
        } else {
            warn!("filter using hue from gray color is not supported");
        }
    }

    indexes
}

fn filter_by_lightness(
    mut indexes: Vec<u8>,
    lightness_level: u8,
    brighten: Option<bool>,
    colors: &Ansi256QuantizedColors,
) -> Vec<u8> {
    // Translate cli option "brighten" to operator to be used for filtering.
    let op = match brighten {
        None => <u8 as PartialEq>::eq,
        Some(true) => <u8 as PartialOrd>::ge,
        Some(false) => <u8 as PartialOrd>::le,
    };

    indexes.retain(|&i| {
        op(
            &colors.color_from_index(i).lightness_level,
            &lightness_level,
        )
    });

    indexes
}

fn filter_grays(mut indexes: Vec<u8>, colors: &Ansi256QuantizedColors) -> Vec<u8> {
    indexes.retain(|&i| colors.color_from_index(i).is_gray());

    indexes
}

fn filter_by_chroma(
    mut indexes: Vec<u8>,
    chroma_level: u8,
    chroma_extent: u8,
    saturate: Option<bool>,
    colors: &Ansi256QuantizedColors,
) -> Vec<u8> {
    let lower_bound = chroma_level.saturating_sub(chroma_extent);
    let upper_bound = chroma_level.saturating_add(chroma_extent);

    let close_to =
        |chroma: u8| PartialOrd::ge(&chroma, &lower_bound) && PartialOrd::le(&chroma, &upper_bound);
    let ge_lower_bound = |chroma: u8| PartialOrd::ge(&chroma, &lower_bound);
    let le_upper_bound = |chroma: u8| PartialOrd::le(&chroma, &upper_bound);

    let f: &dyn Fn(u8) -> bool = match saturate {
        None => &close_to,
        Some(true) => &ge_lower_bound,
        Some(false) => &le_upper_bound,
    };

    indexes.retain(|&i| {
        let chroma = colors.color_from_index(i).chroma;
        f(chroma)
    });

    indexes
}

fn filter_by_hue(
    mut indexes: Vec<u8>,
    mut hue_level: u8,
    hue_extent: u8,
    opposite: bool,
    colors: &Ansi256QuantizedColors,
) -> Vec<u8> {
    // hue level is quantized to 255 levels.
    // hence hue level is wrapped at 0 (or 255)

    if opposite {
        info!("change reference hue to opposite");
        hue_level = hue_level.wrapping_add(128);
    }

    if hue_extent >= 128 {
        info!("reference hue +/- hue_extent should cover full range; skip filtering");
        return indexes;
    }

    // hue is ranged from 0 degree to 360 degree.
    // hue at 360 degreen is close to 1, 2, etc
    // i.e. hue is wrapping at 360 (or 0 degree)

    // to ease comparison, all values are aligned by the shift:
    let shift;
    let shift_op: fn(u8, u8) -> u8;

    if hue_level > hue_extent {
        shift = hue_level - hue_extent;
        shift_op = u8::wrapping_sub;
    } else {
        shift = hue_extent - hue_level;
        shift_op = u8::wrapping_add;
    }
    // the "shift" amount by "shift_op" would shift value of hue_level to hue_extent

    // if hue value after the shift is less than hue_extent x2,
    // it would be close to the reference color
    let double_extent = hue_extent << 1;

    indexes.retain(|&i| {
        let hue = colors.color_from_index(i).hue;
        let shifted_hue = shift_op(hue, shift);

        shifted_hue <= double_extent
    });

    indexes
}

fn dump(
    indexes: &[u8],
    sample: &str,
    background: bool,
    alt_color: Option<u8>,
    ansi256_colors: &Ansi256Colors,
) -> Result<()> {
    let mut foreground_color = DynColors::Ansi(AnsiColors::Default);
    let mut background_color = DynColors::Ansi(AnsiColors::Default);

    if let Some(color_index) = alt_color {
        let alt_color = DynColors::Xterm(XtermColors::from(color_index));

        if background {
            foreground_color = alt_color;
        } else {
            background_color = alt_color;
        }
    }

    let mut stdout = io::stdout().lock();

    for &i in indexes.iter() {
        let xterm_color = DynColors::Xterm(XtermColors::from(i));

        if background {
            background_color = xterm_color;
        } else {
            foreground_color = xterm_color;
        }

        let srgb = ansi256_colors.srgb_from_index(i);
        let oklch = ansi256_colors.oklch_from_index(i);

        writeln!(
            stdout,
            "{:#3} #{:02x}{:02x}{:02x} L: {:.4} chroma: {:.4} hue: {:4.0} {}",
            i,
            srgb.red,
            srgb.green,
            srgb.blue,
            oklch.l,
            oklch.chroma,
            oklch.hue.into_degrees(),
            sample.color(foreground_color).on_color(background_color),
        )?;
    }

    Ok(())
}
