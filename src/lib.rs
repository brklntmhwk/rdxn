/// Calculate the input numerical value in the given base
pub fn fmt_radix<T: Into<f64>>(x: T, base: usize, dp: usize) -> Result<String, String> {
    // return an error message if the given base is invalid
    // the base must be less than 36 (The maximum expressible by 0 to 9 and A to Z)
    if base > 36 || base < 2 {
        return Err(format!("{} is not supported as base.", base));
    }

    // make a char vec for reference
    let table: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // use the `into` method to get a value as `f64` allowing any num types implementing `From<f64>` to come
    let x: f64 = x.into();

    // truncate the fractional part of the input to get the integral part
    // the type assertion further curtails the zero(s) in the fractional part
    let mut integral = x.trunc() as i64;

    // prepare a `String` container for the resultant integral part in the base
    let mut base_int = String::new();

    // divide the integer by the given base until the former becomes less than or equal to the latter
    while integral >= base as i64 {
        // add the remainder to the container
        base_int.push(table[(integral % base as i64) as usize]);

        // divide the integer by the given base and then assign the resultant value to itself
        integral /= base as i64;
    }

    // finally divide the integer by the given base where the former is less than the latter
    base_int.push(table[integral as usize]);

    // add a negative sign to the container if the input is so
    if x.is_sign_negative() {
        base_int.push('-');
    }

    // reverse the order of the resultant integral part and then reassign it to the container
    let base_int = base_int.chars().rev().collect::<String>();

    // get the absolute value of the fractional part
    let mut fract = x.abs().fract();

    // prepare a `String` container for the resultant integral part in the base
    let mut base_fract = String::new();

    // prepare a counter for the following loop
    let mut i = 0;
    loop {
        // multiply the fraction by the given base and then assign the resultant value to itself
        fract *= base as f64;

        //
        base_fract.push(table[fract.trunc() as usize]);

        // increment the counter
        i += 1;

        // break out of the loop if the fractional part becomes 0 or it reaches 15 decimal points
        if fract.fract() == 0. || i > dp {
            break;
        }

        // get the fractional part only again
        fract = fract.fract();
    }

    // return the final result concatenating the integral and fractional parts
    Ok(format!("{}.{}", base_int, base_fract))
}
