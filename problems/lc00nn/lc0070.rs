pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }
    let x: [i32; 2] = [1, 2];
    let a: [i32; 4] = [0, 1, 1, 1];

    /// Matrix multiplication
    fn mul(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
        [
            a[0] * b[0] + a[1] * b[2],
            a[0] * b[1] + a[1] * b[3],
            a[2] * b[0] + a[3] * b[2],
            a[2] * b[1] + a[3] * b[3],
        ]
    }

    /// Matrix multiplication
    fn mulv(a: &[i32; 4], x: &[i32; 2]) -> [i32; 2] {
        [a[0] * x[0] + a[1] * x[1], a[2] * x[0] + a[3] * x[1]]
    }

    fn quick_pow(a: &[i32; 4], mut p: u8) -> [i32; 4] {
        let mut multiplier = a.clone();
        let mut ans = [1, 0, 0, 1];
        while p > 0 {
            if p % 2 == 1 {
                ans = mul(&multiplier, &ans);
            }
            multiplier = mul(&multiplier, &multiplier);
            p /= 2;
        }
        ans
    }
    let m = quick_pow(&a, n as u8 - 2);
    mulv(&m, &x)[1]
}
