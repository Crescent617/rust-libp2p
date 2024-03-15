#[macro_export]
macro_rules! concat_str {
    ($($s: expr),+) => {{
        const STRS: &[&str] = &[
            $($s,)+
        ];

        const TOTAL_LEN: usize = {
            let mut ans = 0;
            let mut arr = STRS;
            while let [x, xs @ ..] = arr {
                ans += x.len();
                arr = xs;
            }
            ans
        };

        const CONST_STR_BUF: [u8; TOTAL_LEN] = {
            let mut buf: [u8; TOTAL_LEN] = [0; TOTAL_LEN];
            let mut cur: usize = 0;
            let mut arr = STRS;
            while let [x, xs @ ..] = arr {
                let bytes = x.as_bytes();
                let mut i = 0;
                while i < bytes.len() {
                    buf[cur] = bytes[i];
                    i += 1;
                    cur += 1;
                }
                arr = xs;
            }
            buf
        };

        unsafe { ::core::str::from_utf8_unchecked(&CONST_STR_BUF) }
    }};
}
