use super::Canvas;
use crate::visuals::Color;

const MIN_PX_VAL: f64 = 0.0;
const MAX_PX_VAL: f64 = 255.0;
// typical cutoff for ppm file length is 70,
// keep at 69 and divide by three to keep the RGB
// groupings of 3 nice
const PPM_LINE_LEN: usize = 70;

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut ppm_str = format!("P3\n{} {}\n255\n", canvas.grid[0].len(), canvas.grid.len());
    let num_rows = canvas.grid.len();
    let num_cols = canvas.grid[0].len();

    let mut lin_split;
    for i in 0..num_rows {
        lin_split = 0;
        for j in 0..num_cols {
            let entry = color_format(&canvas.grid[i][j]);
            // colors come in groups of 3 (RGB), count them as one group
            let entry_chars_count = entry.chars().count();
            lin_split += entry_chars_count;

            if lin_split > PPM_LINE_LEN {
                // pop off trailing space
                ppm_str.pop();
                // Newline for the overflow size
                ppm_str.push('\n');

                // reset line split, account for chars added to new line
                lin_split = entry_chars_count;
            }

            ppm_str.push_str(entry.as_str());
            // add space after
            ppm_str.push(' ');
        }

        // pop off trailing space
        ppm_str.pop();
        ppm_str.push('\n');
    }

    ppm_str
}

// ppm format we're using expects colors to be scaled to lie
// between 0 - 255
fn val_normalize(val: f64) -> usize {
    let mut scaled_val = val * MAX_PX_VAL;
    if scaled_val < MIN_PX_VAL {
        scaled_val = MIN_PX_VAL;
    } else if scaled_val > MAX_PX_VAL {
        scaled_val = MAX_PX_VAL;
    }

    scaled_val.round() as usize
}

fn color_format(c: &Color) -> String {
    format!(
        "{} {} {}",
        val_normalize(c.0),
        val_normalize(c.1),
        val_normalize(c.2)
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::new(3, 5);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(1, 2, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(2, 4, Color::new(-0.5, 0.0, 1.0));

        let ppm_str = canvas_to_ppm(&canvas);
        let str_parts: Vec<&str> = ppm_str.split("\n").collect();

        // There should be enough data for at least the headers
        assert_eq!(str_parts.len(), 9);
        assert_eq!(str_parts[0], "P3");
        assert_eq!(
            str_parts[1],
            format!("{} {}", canvas.grid[0].len(), canvas.grid.len())
        );
        assert_eq!(str_parts[2], "255");
        assert_eq!(str_parts[3], "255 0 0 0 0 0 0 0 0");
        assert_eq!(str_parts[4], "0 0 0 0 0 0 0 0 0");
        assert_eq!(str_parts[5], "0 0 0 0 128 0 0 0 0");
        assert_eq!(str_parts[6], "0 0 0 0 0 0 0 0 0");
        assert_eq!(str_parts[7], "0 0 0 0 0 0 0 0 255");
        assert_eq!(str_parts[8], "");
    }

    #[test]
    fn ppm_split_lines() {
        let num_rows = 10;
        let num_cols = 2;
        let mut canvas = Canvas::new(num_rows, num_cols);

        for i in 0..num_rows {
            for j in 0..num_cols {
                canvas.write_pixel(i, j, Color::new(1.0, 0.8, 0.6));
            }
        }

        let ppm_str = canvas_to_ppm(&canvas);
        let str_parts: Vec<&str> = ppm_str.split("\n").collect();

        assert_eq!(
            str_parts[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            str_parts[4],
            "255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            str_parts[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            str_parts[6],
            "255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }
}
