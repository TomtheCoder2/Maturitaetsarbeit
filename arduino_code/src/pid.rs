#![no_std]

use com::FixedPoint;

pub struct PIDController {
    pub kp: FixedPoint,     // Proportional gain
    pub ki: FixedPoint,     // Integral gain
    pub kd: FixedPoint,     // Derivative gain
    integral: FixedPoint,   // Integral of the error
    last_error: FixedPoint, // Last error
}

impl PIDController {
    pub fn new(kp: i32, ki: i32, kd: i32, frac_bits: u8) -> Self {
        PIDController {
            kp: FixedPoint::from_raw(kp, frac_bits),
            ki: FixedPoint::from_raw(ki, frac_bits),
            kd: FixedPoint::from_raw(kd, frac_bits),
            integral: FixedPoint::new(0, frac_bits),
            last_error: FixedPoint::new(0, frac_bits),
        }
    }

    pub fn update(&mut self, error: i32, dt: i32) -> i32 {
        let error_fp = FixedPoint::new(error, self.kp.frac_bits);
        let dt_fp = FixedPoint::from_raw(dt, self.kp.frac_bits);

        self.integral = self.integral.add(error_fp.mul(dt_fp));
        let derivative = error_fp.sub(self.last_error).div(dt_fp);

        self.last_error = error_fp;

        let output = self
            .kp
            .mul(error_fp)
            .add(self.ki.mul(self.integral))
            .add(self.kd.mul(derivative));

        output.to_i32()
    }

    pub fn update_count(&mut self, error: i32, dt: i32, count: i32) -> i32 {
        let error_fp = FixedPoint::new(error, self.kp.frac_bits);
        let dt_fp = FixedPoint::new(dt, self.kp.frac_bits);
        let count_fp = FixedPoint::new(count, self.kp.frac_bits);

        self.integral = self.integral.add(error_fp.mul(dt_fp)).div(count_fp);
        let derivative = error_fp.sub(self.last_error).div(dt_fp);

        self.last_error = error_fp;

        let output = self
            .kp
            .mul(error_fp)
            .add(self.ki.mul(self.integral))
            .add(self.kd.mul(derivative));

        output.to_i32()
    }
}
