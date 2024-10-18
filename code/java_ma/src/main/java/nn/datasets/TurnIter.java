package nn.datasets;

import static java.lang.Math.*;
import static nn.datasets.Turn.*;
public class TurnIter {
    final int n_integral = 10;
    int v_end = 250;
    int v_start = 175;
    int v_max = 1000;
    float dist = 0;
    float driven_distance = 0;
    int speed = 0;
    float break_percentage = 0.8f;
    float break_m;
    float break_c;
    float acc_percentage = 0.2f;
    float acc_m;
    float acc_c = v_start;
    float k_p_sync = 7.5f,
            k_i_sync = 5.0f,
            k_d_sync = 5.0f,
            beta_sync = 1.0f;
    int left_average_speed = 0,
            right_average_speed = 0;
    float[] integral_sync = new float[n_integral];
    float last_error_sync = 0f;

    public static final float wheel_distance = 190.0f;
    public static final float wheel_diameter = 62.0f;

    public TurnIter(float dist) {
        this.dist = dist;
        acc_m = ((float) (v_max - v_start)) / (acc_percentage * dist);
        break_m = (float) (((float) (v_end - v_max)) / ((1.0 - break_percentage) * dist));
        break_c = v_max - break_m * (dist * break_percentage);
    }

    public TurnIter(int angle) {
        dist = (float) ((wheel_distance) * PI * ((float) angle) / 360.0f);
//        System.out.println("dist: " + dist);
        acc_m = ((float) (v_max - v_start)) / (acc_percentage * dist);
        break_m = (float) (((float) (v_end - v_max)) / ((1.0 - break_percentage) * dist));
        break_c = v_max - break_m * (dist * break_percentage);
    }

    public Turn.Pair<Integer, Integer> turn(int c, int left_current_speed, int right_current_speed, float left_distance, float right_distance) {
        if (driven_distance > dist) {
            stop = true;
            return new Turn.Pair<>(0, 0);
        }
        int real_speed = (left_current_speed + right_current_speed) / 2;
        speed = real_speed;
        driven_distance = abs((left_distance + right_distance) / 2);
        if (driven_distance > dist * break_percentage) {
            speed = (int) (break_m * driven_distance + break_c);
        } else if (speed != v_max) {
            speed = min(v_max, (int) (acc_m * driven_distance + acc_c));
        }
        if (speed < min(min(v_start, v_max), v_end)) {
            speed = min(min(v_start, v_max), v_end);
        }

        if (driven_distance > dist) {
            stop = true;
            return new Turn.Pair<>(0, 0);
        }

        left_average_speed = (left_average_speed * c + left_current_speed) / (c + 1);
        right_average_speed = (right_average_speed * c + right_current_speed) / (c + 1);

        float error_sync = 0;
        if (abs(left_average_speed) > 100 && abs(right_average_speed) > 100) {
            error_sync = left_average_speed - right_average_speed;
        }
        integral_sync[c % n_integral] = error_sync;

        float integral_sum_sync = 0;
        for (float v : integral_sync) {
            integral_sum_sync += v;
        }
        integral_sum_sync /= n_integral;
        float derivative_sync = error_sync - last_error_sync;
        float output_sync =
                k_p_sync * error_sync +
                        k_i_sync * integral_sum_sync +
                        k_d_sync * derivative_sync;
        output_sync *= beta_sync;
        if (!(abs(left_average_speed) > 100 && abs(right_average_speed) > 100 && abs(output_sync) < 500)) {
            output_sync = 0;
        }
        int left_speed = (int) (speed - output_sync);
        int right_speed = (int) (speed + output_sync);
        // cant be greater than 1400
        // cant be smaller than -1400
        left_speed = min(1400, max(-1400, left_speed));
        right_speed = min(1400, max(-1400, right_speed));
        last_error_sync = error_sync;
//        System.out.println("left_speed: " + left_speed + " right_speed: " + right_speed);
        return new Turn.Pair<>(left_speed, right_speed);
    }

}
