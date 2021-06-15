extern crate bmp;
extern crate clap;

use bmp::{Image, Pixel};
use clap::{App, Arg};

#[derive(Debug)]
enum PcbSetup {
    Coplanar(CoplanarSetup),
    Via(ViaSetup),
}

#[derive(Debug)]
struct CoplanarSetup {
    res_x: u32,
    res_y: u32,
    core_thickness: u32,
    cu_thickness: u32,
    sm_thickness: u32,
    trace_width: u32,
    outer_space: u32,
    inner_space: u32,
    via_fence_dist: u32,
    via_fence_thickness: u32,
    filename: String,
}

#[derive(Debug)]
struct ViaSetup {
    res_x: u32,
    res_y: u32,
    inner_width: u32,
    outer_width: u32,
    inner_space: u32,
    outer_space: u32,
    filename: String,
}

const AIR_COLOR: Pixel = Pixel {
    r: 255,
    g: 202,
    b: 202,
};
const FR4_COLOR: Pixel = Pixel {
    r: 223,
    g: 247,
    b: 136,
};
const POS_COLOR: Pixel = Pixel { r: 255, g: 0, b: 0 };
const NEG_COLOR: Pixel = Pixel { r: 0, g: 0, b: 255 };
const GND_COLOR: Pixel = Pixel { r: 0, g: 255, b: 0 };
const SMK_COLOR: Pixel = Pixel {
    r: 25,
    g: 186,
    b: 246,
};

fn draw_circle(
    img: &mut Image,
    color: Pixel,
    x_pos: u32,
    y_pos: u32,
    radius: u32,
) {
    let x_pos = x_pos as i32;
    let y_pos = y_pos as i32;
    let radius = radius as i32;

    let mut err: i32 = 1 - radius as i32;
    let mut dx: i32 = 0;
    let mut dy: i32 = -2 * radius as i32;
    let mut x: i32 = 1;
    let mut y: i32 = radius as i32;

    img.set_pixel((x_pos + radius) as u32, y_pos as u32, color);
    img.set_pixel((x_pos - radius) as u32, y_pos as u32, color);
    img.set_pixel(x_pos as u32, (y_pos + radius) as u32, color);
    img.set_pixel(x_pos as u32, (y_pos - radius) as u32, color);
    fill_column(
        img,
        color,
        x_pos as u32,
        (y_pos - radius + 1) as u32,
        (y_pos + radius) as u32,
    );

    while x < y {
        if err >= 0 {
            y -= 1;
            dy += 2;
            err += dy;
        }
        dx += 2;
        err += dx + 1;

        img.set_pixel((x_pos + x) as u32, (y_pos + y) as u32, color);
        img.set_pixel((x_pos - x) as u32, (y_pos + y) as u32, color);
        img.set_pixel((x_pos + x) as u32, (y_pos - y) as u32, color);
        img.set_pixel((x_pos - x) as u32, (y_pos - y) as u32, color);
        img.set_pixel((x_pos + y) as u32, (y_pos + x) as u32, color);
        img.set_pixel((x_pos - y) as u32, (y_pos + x) as u32, color);
        img.set_pixel((x_pos + y) as u32, (y_pos - x) as u32, color);
        img.set_pixel((x_pos - y) as u32, (y_pos - x) as u32, color);
        fill_column(
            img,
            color,
            (x_pos + x) as u32,
            (y_pos - y + 1) as u32,
            (y_pos + y) as u32,
        );
        fill_column(
            img,
            color,
            (x_pos + y) as u32,
            (y_pos - x + 1) as u32,
            (y_pos + x) as u32,
        );
        fill_column(
            img,
            color,
            (x_pos - x) as u32,
            (y_pos - y + 1) as u32,
            (y_pos + y) as u32,
        );
        fill_column(
            img,
            color,
            (x_pos - y) as u32,
            (y_pos - x + 1) as u32,
            (y_pos + x) as u32,
        );
        x += 1;
    }
}

fn fill_column(
    img: &mut Image,
    color: Pixel,
    x: u32,
    y_start: u32,
    y_end: u32,
) {
    for y in y_start..y_end {
        img.set_pixel(x, y, color);
    }
}

impl PcbSetup {
    fn from_args() -> PcbSetup {
        let matches = App::new("atlc-gen")
            .version("0.1")
            .arg(
                Arg::with_name("Coplanar Setup")
                    .long("--coplanar")
                    .conflicts_with("Via Setup")
                    .required_unless("Via Setup"),
            )
            .arg(
                Arg::with_name("Via Setup")
                    .long("--via")
                    .conflicts_with("Coplanar Setup")
                    .required_unless("Coplanar Setup"),
            )
            .arg(
                Arg::with_name("resolution")
                    .short("r")
                    .long("resolution")
                    .takes_value(true),
            )
            .arg(Arg::with_name("res_x").short("x").takes_value(true))
            .arg(Arg::with_name("res_y").short("y").takes_value(true))
            .arg(
                Arg::with_name("core-thickness")
                    .short("C")
                    .long("core-thickness")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("cu-thickness")
                    .short("c")
                    .long("cu-thickness")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("sm-thickness")
                    .short("m")
                    .long("sm-thickness")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("trace-width")
                    .short("t")
                    .long("trace-width")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("outer-space")
                    .short("s")
                    .long("outer-space")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("via-fence-dist")
                    .short("v")
                    .long("via-fence-dist")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("via-thickness")
                    .short("V")
                    .long("via-thickness")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("inner-space")
                    .short("S")
                    .long("inner-space")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("filename")
                    .short("f")
                    .long("out-filename")
                    .takes_value(true),
            )
            .get_matches();

        let resolution: u32 = match matches.value_of("resolution") {
            Some(x) => x.parse().unwrap(),
            None => 5,
        };
        let res_x: u32 = match matches.value_of("res_x") {
            Some(x) => x.parse().unwrap(),
            None => 10000,
        };
        let res_y: u32 = match matches.value_of("res_y") {
            Some(x) => x.parse().unwrap(),
            None => 7500,
        };
        let core_thickness: u32 = match matches.value_of("core-thickness") {
            Some(x) => x.parse().unwrap(),
            None => 1500,
        };
        let cu_thickness: u32 = match matches.value_of("cu-thickness") {
            Some(x) => x.parse().unwrap(),
            None => 35,
        };
        let sm_thickness: u32 = match matches.value_of("sm-thickness") {
            Some(x) => x.parse().unwrap(),
            None => 10,
        };
        let trace_width: u32 = match matches.value_of("trace-width") {
            Some(x) => x.parse().unwrap(),
            None => 200,
        };
        let outer_space: u32 = match matches.value_of("outer-space") {
            Some(x) => x.parse().unwrap(),
            None => 2000,
        };
        let inner_space: u32 = match matches.value_of("inner-space") {
            Some(x) => x.parse().unwrap(),
            None => 200,
        };
        let via_fence_dist: u32 = match matches.value_of("via-fence-dist") {
            Some(x) => x.parse().unwrap(),
            None => 200,
        };
        let via_thickness: u32 = match matches.value_of("via-thickness") {
            Some(x) => x.parse().unwrap(),
            None => 300,
        };
        let filename: String = match matches.value_of("out-filename") {
            Some(x) => x.to_string(),
            None => "atlc-gen.bmp".to_string(),
        };

        if matches.is_present("Coplanar Setup") {
            return PcbSetup::Coplanar(CoplanarSetup {
                res_x: res_x / resolution,
                res_y: res_y / resolution,
                core_thickness: core_thickness / resolution,
                cu_thickness: cu_thickness / resolution,
                sm_thickness: sm_thickness / resolution,
                trace_width: trace_width / resolution,
                outer_space: outer_space / resolution,
                inner_space: inner_space / resolution,
                via_fence_dist: via_fence_dist / resolution,
                via_fence_thickness: via_thickness / resolution,
                filename: filename,
            });
        } else if matches.is_present("Via Setup") {
            return PcbSetup::Via(ViaSetup {
                res_x: res_x / resolution,
                res_y: res_y / resolution,
                inner_width: trace_width / resolution,
                outer_width: via_thickness / resolution,
                outer_space: outer_space / resolution,
                inner_space: inner_space / resolution,
                filename: filename,
            });
        }
        panic!("Can never happen");
    }
}

impl CoplanarSetup {
    fn to_bitmap(&self) -> Image {
        let mut img = Image::new(self.res_x, self.res_y);

        // Fill with air.
        for (x, y) in img.coordinates() {
            img.set_pixel(x, y, AIR_COLOR);
        }

        // Groundplane
        for x in 0..self.res_x {
            for y in 0..self.cu_thickness {
                img.set_pixel(x, self.res_y - 1 - y, GND_COLOR);
            }
        }

        let top_gnd_spacing = if self.inner_space == 0 {
            self.trace_width + 2 * self.outer_space
        } else {
            2 * self.trace_width + self.inner_space + 2 * self.outer_space
        };

        // FR4 and fence
        let fence_x1 = (self.res_x - top_gnd_spacing) / 2 - self.via_fence_dist;
        let fence_x2 = (self.res_x + top_gnd_spacing) / 2 + self.via_fence_dist;
        let fence_range1 = fence_x1 - self.via_fence_thickness..fence_x1;
        let fence_range2 = fence_x2..fence_x2 + self.via_fence_thickness;
        for y in self.cu_thickness..self.cu_thickness + self.core_thickness {
            for x in 0..self.res_x {
                if fence_range1.contains(&x) || fence_range2.contains(&x) {
                    img.set_pixel(x, self.res_y - 1 - y, GND_COLOR);
                } else {
                    img.set_pixel(x, self.res_y - 1 - y, FR4_COLOR);
                }
            }
        }

        // Top layer
        for y in self.cu_thickness + self.core_thickness
            ..2 * self.cu_thickness + self.core_thickness
        {
            for x in 0..(self.res_x - top_gnd_spacing) / 2 {
                img.set_pixel(x, self.res_y - 1 - y, GND_COLOR);
            }
            for x in (self.res_x + top_gnd_spacing) / 2..self.res_x {
                img.set_pixel(x, self.res_y - 1 - y, GND_COLOR);
            }

            if self.inner_space == 0 {
                for x in (self.res_x - self.trace_width) / 2
                    ..(self.res_x + self.trace_width) / 2
                {
                    img.set_pixel(x, self.res_y - 1 - y, POS_COLOR);
                }
            } else {
                for x in (self.res_x - self.inner_space) / 2 - self.trace_width
                    ..(self.res_x - self.inner_space) / 2
                {
                    img.set_pixel(x, self.res_y - 1 - y, NEG_COLOR);
                }
                for x in (self.res_x + self.inner_space) / 2
                    ..(self.res_x + self.inner_space) / 2 + self.trace_width
                {
                    img.set_pixel(x, self.res_y - 1 - y, POS_COLOR);
                }
            }
        }

        // Soldermask by dilation
        for y in self.cu_thickness + self.core_thickness
            ..2 * self.cu_thickness + self.core_thickness + self.sm_thickness
        {
            for x in 0..self.res_x {
                if ((-(self.sm_thickness as i32))
                    ..(self.sm_thickness as i32) + 1)
                    .any(|i| {
                        let xd = ((x as i32) + i) as u32;
                        let yd = self.res_y - 1 - (y - self.sm_thickness);
                        if (0..self.res_x).contains(&xd)
                            && (0..self.res_y).contains(&yd)
                        {
                            return img.get_pixel(xd, yd) != AIR_COLOR
                                && img.get_pixel(xd, yd) != SMK_COLOR
                                && img.get_pixel(x, self.res_y - 1 - y)
                                    == AIR_COLOR;
                        }
                        return false;
                    })
                {
                    img.set_pixel(x, self.res_y - 1 - y, SMK_COLOR);
                }
            }
        }

        return img;
    }
}

impl ViaSetup {
    fn to_bitmap(&self) -> Image {
        let mut img = Image::new(self.res_x, self.res_y);

        // Fill with FR4.
        for (x, y) in img.coordinates() {
            img.set_pixel(x, y, FR4_COLOR);
        }

        let center_x = self.res_x / 2;
        let center_y = self.res_y / 2;
        let conductor_pos = self.inner_space / 2 + self.inner_width / 2;

        // Conductors
        if self.inner_space == 0 {
            draw_circle(
                &mut img,
                POS_COLOR,
                center_x,
                center_y,
                self.inner_width / 2,
            );
        } else {
            draw_circle(
                &mut img,
                POS_COLOR,
                center_x + conductor_pos,
                center_y,
                self.inner_width / 2,
            );
            draw_circle(
                &mut img,
                NEG_COLOR,
                center_x - conductor_pos,
                center_y,
                self.inner_width / 2,
            );
        }

        // GND vias
        if self.inner_space == 0 {
            let offset =
                (1.0 / 2_f32.sqrt() * self.outer_space as f32).round() as u32;
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x + conductor_pos + offset,
                center_y + offset,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x - conductor_pos - offset,
                center_y + offset,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x + conductor_pos + offset,
                center_y - offset,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x - conductor_pos - offset,
                center_y - offset,
                self.outer_width / 2,
            );
        } else {
            let offset_x = self.outer_space / 2;
            let offset_y =
                ((self.outer_space / 2) as f32 * 3_f32.sqrt()).round() as u32;
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x + conductor_pos + self.outer_space,
                center_y,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x - conductor_pos - self.outer_space,
                center_y,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x + conductor_pos + offset_x,
                center_y + offset_y,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x - conductor_pos - offset_x,
                center_y + offset_y,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x + conductor_pos + offset_x,
                center_y - offset_y,
                self.outer_width / 2,
            );
            draw_circle(
                &mut img,
                GND_COLOR,
                center_x - conductor_pos - offset_x,
                center_y - offset_y,
                self.outer_width / 2,
            );
        }

        return img;
    }
}

fn main() {
    let settings = PcbSetup::from_args();
    println!("Generating atlc bitmat with {:?}", settings);
    match settings {
        PcbSetup::Coplanar(settings) => {
            let bitmap = settings.to_bitmap();
            bitmap.save(settings.filename).unwrap();
        }
        PcbSetup::Via(settings) => {
            let bitmap = settings.to_bitmap();
            bitmap.save(settings.filename).unwrap();
        }
    }
}
