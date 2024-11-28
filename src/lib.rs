pub fn fmt_radix<T: Into<f64>>(x: T, base: usize) -> Result<String, String> {
    if base > 36 || base < 2 {
        return Err(format!("{} is not supported as base.", base));
    }

    let table: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let x: f64 = x.into();

    let mut integral = x.trunc() as i64;
    let mut base_int = String::new();

    while integral >= base as i64 {
        base_int.push(table[(integral % base as i64) as usize]);
        integral /= base as i64;
    }

    base_int.push(table[integral as usize]);

    if x.is_sign_negative() {
        base_int.push('-');
    }

    let base_int = base_int.chars().rev().collect::<String>();

    let mut fract = x.abs().fract();
    let mut base_fract = String::new();
    let mut i = 0;

    loop {
        fract *= base as f64;
        base_fract.push(table[fract.trunc() as usize]);
        i += 1;
        if fract.fract() == 0. || i >= 15 {
            break;
        }
        fract = fract.fract();
    }

    Ok(format!("{}.{}", base_int, base_fract))
}
