pub fn negative_spell(n: i64) -> String {
    const BELOW_20: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    const THOUSANDS: [&str; 7] = [
        "", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
    ];

    fn spell(n: u64) -> String {
        if n < 20 {
            return BELOW_20[n as usize].to_string();
        } else if n < 100 {
            let tens = n / 10;
            let rest = n % 10;
            return if rest == 0 {
                TENS[tens as usize].to_string()
            } else {
                format!("{}-{}", TENS[tens as usize], BELOW_20[rest as usize])
            };
        } else if n < 1000 {
            let hundreds = n / 100;
            let rest = n % 100;
            return if rest == 0 {
                format!("{} hundred", BELOW_20[hundreds as usize])
            } else {
                format!("{} hundred {}", BELOW_20[hundreds as usize], spell(rest))
            };
        } else {
            let mut parts = vec![];
            let mut num = n;
            let mut i = 0;

            while num > 0 {
                let chunk = num % 1000;
                if chunk != 0 {
                    let chunk_str = spell(chunk);
                    let label = THOUSANDS[i];
                    parts.push(if label.is_empty() {
                        chunk_str
                    } else {
                        format!("{} {}", chunk_str, label)
                    });
                }
                num /= 1000;
                i += 1;
            }

            parts.reverse();
            parts.join(" ")
        }
    }

    if n > 0 {
        spell(n as u64)
    } else if n == 0 {
        "zero".to_string()
    } else {
        format!("minus {}", spell((-n) as u64))
    }
}
